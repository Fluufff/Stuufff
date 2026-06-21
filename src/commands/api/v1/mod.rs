use axum::{Router, routing::get};

use super::ApiState;

pub mod auth;

pub fn init() -> Router<ApiState> {
    Router::new()
        .route("/auth/whoami", get(auth::routes::whoami))
        .route("/auth/google_cb", get(auth::routes::google_cb))
        .route("/auth/logout", get(auth::routes::logout))
}
