use axum::{Router, routing::get};

use super::ApiState;

pub mod auth;
mod routes;

pub fn init() -> Router<ApiState> {
    Router::new()
        .route("/auth/whoami", get(auth::routes::whoami))
        .route("/auth/google_cb", get(auth::routes::google_cb))
        .route("/auth/logout", get(auth::routes::logout))
        .route("/things", get(routes::list_things))
        .route("/things/{id}", get(routes::get_thing))
}
