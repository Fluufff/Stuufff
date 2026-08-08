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
pub struct ApiDepartment {
    #[serde(flatten)]
    pub department: models::Department,
    pub image_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiNewDepartment {
    pub name: String,
}

pub async fn list_departments(State(state): State<ApiState>, headers: HeaderMap) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let departments = match schema::departments::table
        .select(models::Department::as_select())
        .load(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get departments");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = match models::DepartmentImage::belonging_to(&departments)
        .select(models::DepartmentImage::as_select())
        .load::<models::DepartmentImage>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get images");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = images.grouped_by(&departments);

    let result = departments
        .into_iter()
        .zip(images)
        .map(|(department, images)| ApiDepartment {
            department,
            image_ids: images.into_iter().map(|ti| ti.id.to_string()).collect(),
        })
        .collect::<Vec<_>>();

    Json(result).into_response()
}

pub async fn get_department(
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

    let max_id: i32 = match schema::departments::table
        .select(max(schema::departments::id))
        .get_result(&mut db)
    {
        Ok(Some(max_id)) => max_id,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get department");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    if max_id < id {
        return StatusCode::NOT_FOUND.into_response();
    }

    let department = match schema::departments::table
        .find(&id)
        .first::<models::Department>(&mut db)
        .optional()
    {
        Ok(Some(department)) => department,
        Ok(None) => return StatusCode::GONE.into_response(),
        Err(e) => {
            error!(err = e.to_string(), "failed to get department");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let images = match models::DepartmentImage::belonging_to(&department)
        .select(models::DepartmentImage::as_select())
        .load::<models::DepartmentImage>(&mut db)
    {
        Ok(res) => res,
        Err(e) => {
            error!(err = e.to_string(), "failed to get images");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    let result = ApiDepartment {
        department,
        image_ids: images.into_iter().map(|ti| ti.id.to_string()).collect(),
    };

    Json(result).into_response()
}

pub async fn add_department(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(thing): Json<ApiNewDepartment>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let id = match diesel::insert_into(schema::departments::table)
        .values(models_new::NewDepartment {
            name: &thing.name,
            main_img: None,
        })
        .returning(schema::departments::id)
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

pub async fn update_department(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(department): Json<ApiDepartment>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    if id != department.department.id {
        return (StatusCode::BAD_REQUEST, "id cannot be changed").into_response();
    }

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::update(schema::departments::table.filter(schema::departments::id.eq(id)))
        .set(department.department)
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    StatusCode::OK.into_response()
}

pub async fn delete_department(
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

    match diesel::delete(schema::departments::table.filter(schema::departments::id.eq(id)))
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    StatusCode::OK.into_response()
}

pub async fn get_department_images(
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

    let images = match schema::department_images::table
        .filter(schema::department_images::department_id.eq(&id))
        .load::<models::DepartmentImage>(&mut db)
    {
        Ok(images) => images,
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(images).into_response()
}

pub async fn get_department_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((department_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::READER) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let _img = match schema::department_images::table
        .filter(
            schema::department_images::id
                .eq(img_id)
                .and(schema::department_images::department_id.eq(department_id)),
        )
        .first::<models::DepartmentImage>(&mut db)
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

pub async fn delete_department_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path((department_id, img_id)): Path<(i32, Uuid)>,
) -> Response {
    let _auth = match AuthInfo::minimum(&headers, &state.config, &Authorization::EDITOR) {
        Ok(auth) => auth,
        Err(resp) => return resp,
    };

    let mut db = match state.config.psql_pool.connection_or_response() {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    match diesel::delete(schema::department_images::table)
        .filter(
            schema::department_images::id
                .eq(img_id)
                .and(schema::department_images::department_id.eq(department_id)),
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

pub async fn add_department_image(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Path(department_id): Path<i32>,
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

    match insert_into(schema::department_images::table)
        .values(&models::DepartmentImage {
            department_id,
            id: uuid,
        })
        .execute(&mut db)
    {
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
        _ => {}
    };

    let main_img = match schema::departments::table
        .find(department_id)
        .select(schema::departments::main_img)
        .first::<Option<Uuid>>(&mut db)
    {
        Ok(id) => id,
        Err(e) => return (StatusCode::SERVICE_UNAVAILABLE, format!("{e}")).into_response(),
    };

    if main_img.is_none() {
        match diesel::update(schema::departments::table.find(department_id))
            .set(schema::departments::main_img.eq(uuid))
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
