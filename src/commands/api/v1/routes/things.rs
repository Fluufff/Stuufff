use std::{collections::HashSet, io, pin::pin};

use crate::psql::{models, models_new, schema};

use super::super::ApiState;
use super::super::auth::{AuthInfo, Authorization};
use axum::{
    Json,
    extract::{Path, Query, Request, State},
    response::{IntoResponse, Response},
};
use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, OptionalExtension,
    PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, insert_into,
};
use futures::TryStreamExt;
use http::{HeaderMap, StatusCode};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as};
use tokio::{
    fs::{self, File},
    io::BufWriter,
};
use tokio_util::io::StreamReader;
use tracing::error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ApiNewLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiNewThing {
    pub count: Option<i32>,
    pub name: String,
    pub description: String,
    pub in_place: Option<i32>,
    pub in_department: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiThing {
    #[serde(flatten)]
    pub thing: models::Thing,
    pub label_ids: Vec<i32>,
    pub reservations: Vec<ApiReservation>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiThingDetailed {
    #[serde(flatten)]
    pub thing: models::Thing,
    pub department: Option<i32>,
    pub label_ids: Vec<i32>,
    pub image_ids: Vec<String>,
    pub reservations: Vec<ApiReservation>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiReservation {
    pub id: i32,
    pub count: i32,
    pub reserved_by: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ThingsListFilters {
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    search: Option<String>,

    #[serde(default)]
    place: Option<i32>,
}

pub async fn list_things(
    filters: Query<ThingsListFilters>,
    State(state): State<ApiState>,
    headers: HeaderMap,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let mut query = schema::things::table
        // .left_join(schema::places::table)
        // .select((models::Thing::as_select(), schema::places::name.nullable()))
        .select(models::Thing::as_select())
        .into_boxed();

    if let Some(search) = filters.search.as_ref() {
        let s = format!("%{search}%");
        query = query.filter(
            schema::things::name
                .ilike(s.clone())
                .or(schema::things::description.ilike(s)),
        )
    }

    match filters.place.as_ref() {
        None => {}
        Some(0) => query = query.filter(schema::things::in_place.is_null()),
        Some(id) => query = query.filter(schema::things::in_place.eq(id)),
    }

    let things = match query.load::<models::Thing>(&mut db) {
        // let things = match query.load::<(models::Thing, Option<String>)>(&mut db) {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    // let labels = match models::ThingLabel::belonging_to(&things)
    //     .inner_join(schema::labels::table)
    //     .select((models::ThingLabel::as_select(), models::Label::as_select()))
    //     .load::<(models::ThingLabel, models::Label)>(&mut db)
    // {
    //     Ok(things) => things,
    //     Err(e) => {
    //         error!(err = e.to_string(), "failed to get things");
    //         return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
    //     }
    // };

    let labels = match models::ThingLabel::belonging_to(&things)
        .select(models::ThingLabel::as_select())
        .load::<models::ThingLabel>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let reservations = match models::ReservedThing::belonging_to(&things)
        .select(models::ReservedThing::as_select())
        .load::<models::ReservedThing>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let labels = labels.grouped_by(&things);
    let reservations = reservations.grouped_by(&things);

    let result = things
        .into_iter()
        .zip(labels)
        .zip(reservations)
        .map(|((thing, labels), reservations)| ApiThing {
            thing,
            label_ids: labels.into_iter().map(|tl| tl.label_id).collect(),
            reservations: reservations
                .into_iter()
                .map(|r| ApiReservation {
                    id: r.id,
                    count: r.count,
                    reserved_by: r.reserved_by,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    Json(result).into_response()
}

pub async fn get_thing(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let thing = match schema::things::table
        .find(&id)
        .first::<models::Thing>(&mut db)
        .optional()
    {
        Ok(Some(thing)) => thing,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = match models::ThingImage::belonging_to(&thing)
        .select(models::ThingImage::as_select())
        .load::<models::ThingImage>(&mut db)
    {
        Ok(images) => images,
        Err(e) => {
            error!(err = e.to_string(), "failed to get images");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let labels = match models::ThingLabel::belonging_to(&thing)
        .select(models::ThingLabel::as_select())
        .load::<models::ThingLabel>(&mut db)
    {
        Ok(labels) => labels,
        Err(e) => {
            error!(err = e.to_string(), "failed to get labels");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let reservations = match models::ReservedThing::belonging_to(&thing)
        .select(models::ReservedThing::as_select())
        .load::<models::ReservedThing>(&mut db)
    {
        Ok(things) => things,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let mut place_to_check_for_dep = thing.in_place.clone();
    let mut department: Option<i32> = None;
    loop {
        let place_id = match place_to_check_for_dep {
            Some(id) => id,
            None => break,
        };

        let (in_dep, in_place) = match schema::places::table
            .filter(schema::places::id.eq(place_id))
            .select((schema::places::in_department, schema::places::in_place))
            .first::<(Option<i32>, Option<i32>)>(&mut db)
        {
            Ok(place) => place,
            Err(e) => {
                error!(err = e.to_string(), "failed to get place");
                return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
            }
        };

        place_to_check_for_dep = in_place;
        if let Some(id) = in_dep {
            department.replace(id);
            break;
        };
    }

    let result = ApiThingDetailed {
        thing,
        department,
        label_ids: labels.into_iter().map(|tl| tl.label_id).collect(),
        image_ids: images.into_iter().map(|ti| ti.id.to_string()).collect(),
        reservations: reservations
            .into_iter()
            .map(|r| ApiReservation {
                id: r.id,
                count: r.count,
                reserved_by: r.reserved_by,
            })
            .collect(),
    };

    Json(result).into_response()
}

pub async fn update_thing(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(thing_id): Path<i32>,
    Json(thing): Json<ApiThing>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    if thing_id != thing.thing.id {
        return (StatusCode::BAD_REQUEST, "id cannot be changed").into_response();
    }

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let labels_in_db = match models::ThingLabel::belonging_to(&thing.thing)
        .select(models::ThingLabel::as_select())
        .load::<models::ThingLabel>(&mut db)
    {
        Ok(labels) => labels,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };
    let labels_in_db = labels_in_db
        .into_iter()
        .map(|l| l.label_id)
        .collect::<HashSet<_>>();
    let labels_in_thing = thing.label_ids.into_iter().collect::<HashSet<_>>();

    match diesel::update(schema::things::table.filter(schema::things::id.eq(thing_id)))
        .set(thing.thing)
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    let labels_to_delete = labels_in_db.difference(&labels_in_thing);
    let labels_to_add = labels_in_thing.difference(&labels_in_db);

    match diesel::delete(
        schema::thing_labels::table.filter(
            schema::thing_labels::thing_id
                .eq(thing_id)
                .and(schema::thing_labels::label_id.eq_any(labels_to_delete)),
        ),
    )
    .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    match diesel::insert_into(schema::thing_labels::table)
        .values(
            labels_to_add
                .into_iter()
                .map(|label_id| models_new::NewThingLabel {
                    thing_id,
                    label_id: *label_id,
                })
                .collect::<Vec<_>>(),
        )
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    StatusCode::OK.into_response()
}

pub async fn add_thing(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(thing): Json<ApiNewThing>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let id = match diesel::insert_into(schema::things::table)
        .values(models_new::NewThing {
            count: thing.count.unwrap_or(1),
            name: &thing.name,
            description: Some(&thing.description),
            in_place: thing.in_place,
            main_img: None,
        })
        .returning(schema::things::id)
        .get_result::<i32>(&mut db)
    {
        Ok(id) => id,
        Err(e) => {
            error!(err = e.to_string(), "failed to insert new thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    #[derive(Serialize)]
    struct Resp {
        id: i32,
    }

    Json(Resp { id }).into_response()
}

pub async fn get_thing_images(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(thing_id): Path<i32>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let thing = match schema::thing_images::table
        .filter(schema::thing_images::thing_id.eq(&thing_id))
        .load::<models::ThingImage>(&mut db)
    {
        Ok(thing) => thing,
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(thing).into_response()
}

pub async fn get_thing_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((thing_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let _img = match schema::thing_images::table
        .filter(
            schema::thing_images::id
                .eq(img_id)
                .and(schema::thing_images::thing_id.eq(thing_id)),
        )
        .first::<models::ThingImage>(&mut db)
        .optional()
    {
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Ok(Some(thing)) => thing,
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    match fs::read(format!("{}/images/{}", state.config.media_folder, img_id)).await {
        Ok(f) => f.into_response(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    }
}

pub async fn delete_thing_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((thing_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::delete(schema::thing_images::table)
        .filter(
            schema::thing_images::id
                .eq(img_id)
                .and(schema::thing_images::thing_id.eq(thing_id)),
        )
        .execute(&mut db)
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    }

    // We do not delete the actual image, in case the action is reverted
    // match fs::remove_file(format!("{}/{}", state.config.media_folder, id)).await {
    //     Ok(f) => StatusCode::OK.into_response(),
    //     Err(e) if e.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
    //     Err(e) => (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    // }
}

pub async fn add_thing_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(thing_id): Path<i32>,
    request: Request,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let uuid = Uuid::now_v7();

    let write_file = async {
        let stream = request.into_body().into_data_stream();
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(io::Error::other);
        let mut body_reader = pin!(StreamReader::new(body_with_io_error));

        // Create the file. `File` implements `AsyncWrite`.
        // let path = std::path::Path::new(state.config.media_folder.as_str()).join(uuid.to_string());
        let path = format!("{}/images/{}", state.config.media_folder, uuid);
        let path = std::path::Path::new(&path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    };
    // .await
    // .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))

    if let Err(err) = write_file.await {
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    }

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match insert_into(schema::thing_images::table)
        .values(&models::ThingImage { thing_id, id: uuid })
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    let main_img = match schema::things::table
        .find(thing_id)
        .select(schema::things::main_img)
        .first::<Option<Uuid>>(&mut db)
    {
        Ok(id) => id,
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    };

    if main_img.is_none() {
        match diesel::update(schema::things::table.find(thing_id))
            .set(schema::things::main_img.eq(uuid))
            .execute(&mut db)
        {
            Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
            _ => {}
        };
    }

    // match fs::read(format!("{}/{}", state.config.media_folder, id)).await {
    //     Ok(f) => f.into_response(),
    //     Err(e) if e.kind() == std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND.into_response(),
    //     Err(e) => (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    // }

    StatusCode::OK.into_response()
}

pub async fn list_labels(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let labels = match schema::labels::table
        .select(models::Label::as_select())
        .load(&mut db)
    {
        Ok(labels) => labels,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(labels).into_response()
}

pub async fn add_label(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(label): Json<ApiNewLabel>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let color = label.color.unwrap_or_else(|| {
        let mut rng = rand::rng();
        let r: u8 = rng.random_range(0..129);
        let g: u8 = rng.random_range(0..129);
        let b: u8 = rng.random_range(0..129);
        format!("#{:x}{:x}{:x}", r, g, b)
    });

    let label = match diesel::insert_into(schema::labels::table)
        .values(models_new::NewLabel {
            name: &label.name,
            description: label.description.as_deref(),
            color: Some(&color),
        })
        .returning(models::Label::as_select())
        .get_result::<models::Label>(&mut db)
    {
        Ok(id) => id,
        Err(e) => {
            error!(err = e.to_string(), "failed to insert new label");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(label).into_response()
}
