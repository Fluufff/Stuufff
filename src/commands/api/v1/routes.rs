use crate::psql::{models, schema};

use super::auth::{AuthInfo, Authorization};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper,
};
use http::{HeaderMap, StatusCode};
use serde::Deserialize;
use serde_with::{NoneAsEmptyString, serde_as};
use tracing::error;

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
    State(state): State<super::ApiState>,
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
        Ok(things) => things,
        Err(e) => {
            error!(err = e.to_string(), "failed to get things");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(things).into_response()
}

pub async fn get_thing(
    State(state): State<super::ApiState>,
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
        Ok(thing) => thing,
        Err(e) => {
            error!(err = e.to_string(), "failed to get thing");
            return (StatusCode::SERVICE_UNAVAILABLE, "db query failed").into_response();
        }
    };

    Json(thing).into_response()
}
