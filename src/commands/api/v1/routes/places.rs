use std::{io, pin::pin};

use crate::psql::{models, models_new, schema};

use super::super::ApiState;
use super::super::auth::{AuthInfo, Authorization};
use axum::{
    Json,
    extract::{Path, Request, State},
    response::{IntoResponse, Response},
};
use diesel::dsl::max;
use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, GroupedBy, OptionalExtension,
    QueryDsl, RunQueryDsl, SelectableHelper, insert_into,
};
use futures::TryStreamExt;
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::fs::create_dir;
use tokio::{
    fs::{self, File},
    io::BufWriter,
};
use tokio_util::io::StreamReader;
use tracing::error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ApiNewPlace {
    pub name: String,
    pub description: String,
    pub in_place: Option<i32>,
    pub in_department: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiPlace {
    #[serde(flatten)]
    pub place: models::Place,
    pub image_ids: Vec<String>,
    pub reservations: Vec<ApiReservation>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiReservation {
    pub id: i32,
    pub reserved_by: String,
}

pub async fn list_places(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let places = match schema::places::table
        .select(models::Place::as_select())
        .load(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get places");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = match models::PlaceImage::belonging_to(&places)
        .select(models::PlaceImage::as_select())
        .load::<models::PlaceImage>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get images");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let reservations = match models::ReservedPlace::belonging_to(&places)
        .select(models::ReservedPlace::as_select())
        .load::<models::ReservedPlace>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get reservations");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = images.grouped_by(&places);
    let reservations = reservations.grouped_by(&places);

    let result = places
        .into_iter()
        .zip(images)
        .zip(reservations)
        .map(|((place, images), reservations)| ApiPlace {
            place,
            image_ids: images.into_iter().map(|ti| ti.id.to_string()).collect(),
            reservations: reservations
                .into_iter()
                .map(|r| ApiReservation {
                    id: r.id,
                    reserved_by: r.reserved_by,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    Json(result).into_response()
}

pub async fn get_place(
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

    let max_id: i32 = match schema::places::table
        .select(max(schema::places::id))
        .get_result(&mut db)
    {
        Ok(Some(max_id)) => max_id,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get place");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    if max_id < id {
        return StatusCode::NOT_FOUND.into_response();
    }

    let place = match schema::places::table
        .find(&id)
        .first::<models::Place>(&mut db)
        .optional()
    {
        Ok(Some(place)) => place,
        Ok(None) => return StatusCode::GONE.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get place");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = match models::PlaceImage::belonging_to(&place)
        .select(models::PlaceImage::as_select())
        .load::<models::PlaceImage>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get images");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let reservations = match models::ReservedPlace::belonging_to(&place)
        .select(models::ReservedPlace::as_select())
        .load::<models::ReservedPlace>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get reservations");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let result = ApiPlace {
        place,
        image_ids: images.into_iter().map(|ti| ti.id.to_string()).collect(),
        reservations: reservations
            .into_iter()
            .map(|r| ApiReservation {
                id: r.id,
                reserved_by: r.reserved_by,
            })
            .collect(),
    };

    Json(result).into_response()
}

pub async fn add_place(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(thing): Json<ApiNewPlace>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let id = match diesel::insert_into(schema::places::table)
        .values(models_new::NewPlace {
            name: &thing.name,
            description: Some(&thing.description),
            in_place: thing.in_place,
            in_department: thing.in_department,
            main_img: None,
        })
        .returning(schema::places::id)
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

pub async fn delete_place(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::delete(schema::places::table.filter(schema::places::id.eq(id))).execute(&mut db) {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    StatusCode::OK.into_response()
}

pub async fn update_place(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(place): Json<ApiPlace>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    if id != place.place.id {
        return (StatusCode::BAD_REQUEST, "id cannot be changed").into_response();
    }

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::update(schema::places::table.filter(schema::places::id.eq(id)))
        .set(place.place)
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    StatusCode::OK.into_response()
}

pub async fn get_place_images(
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

    let images = match schema::place_images::table
        .filter(schema::place_images::place_id.eq(&id))
        .load::<models::PlaceImage>(&mut db)
    {
        Ok(images) => images,
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(images).into_response()
}

pub async fn get_place_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((place_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let _img = match schema::place_images::table
        .filter(
            schema::place_images::id
                .eq(img_id)
                .and(schema::place_images::place_id.eq(place_id)),
        )
        .first::<models::PlaceImage>(&mut db)
        .optional()
    {
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Ok(Some(image)) => image,
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

pub async fn delete_place_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((place_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::delete(schema::place_images::table)
        .filter(
            schema::place_images::id
                .eq(img_id)
                .and(schema::place_images::place_id.eq(place_id)),
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

pub async fn add_place_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(place_id): Path<i32>,
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
        create_dir(format!("{}/images", state.config.media_folder)).await?;
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

    match insert_into(schema::place_images::table)
        .values(&models::PlaceImage { place_id, id: uuid })
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    let main_img = match schema::places::table
        .find(place_id)
        .select(schema::places::main_img)
        .first::<Option<Uuid>>(&mut db)
    {
        Ok(id) => id,
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    };

    if main_img.is_none() {
        match diesel::update(schema::places::table.find(place_id))
            .set(schema::places::main_img.eq(uuid))
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
