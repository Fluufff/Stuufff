use axum::{Router, routing::get};

use super::ApiState;

pub mod auth;
mod routes;

pub fn init() -> Router<ApiState> {
    Router::new()
        .route("/auth/whoami", get(auth::routes::whoami))
        .route("/auth/google_cb", get(auth::routes::google_cb))
        .route("/auth/local", get(auth::routes::local_cb))
        .route("/auth/logout", get(auth::routes::logout))
        .route(
            "/things",
            get(routes::things::list_things).post(routes::things::add_thing),
        )
        .route(
            "/things/{thing_id}",
            get(routes::things::get_thing)
                .put(routes::things::update_thing)
                .delete(routes::things::delete_thing),
        )
        .route(
            "/things/{thing_id}/images",
            get(routes::things::get_thing_images).post(routes::things::add_thing_image),
        )
        .route(
            "/things/{thing_id}/images/{img_id}",
            // get(routes::get_thing_image),
            get(routes::things::get_thing_image).delete(routes::things::delete_thing_image),
        )
        .route(
            "/places",
            get(routes::places::list_places).post(routes::places::add_place),
        )
        .route(
            "/places/{place_id}",
            get(routes::places::get_place)
                .put(routes::places::update_place)
                .delete(routes::places::delete_place),
        )
        .route(
            "/places/{place_id}/images",
            get(routes::places::get_place_images).post(routes::places::add_place_image),
        )
        .route(
            "/places/{place_id}/images/{img_id}",
            // get(routes::get_thing_image),
            get(routes::places::get_place_image).delete(routes::places::delete_place_image),
        )
        .route(
            "/departments",
            get(routes::departments::list_departments).post(routes::departments::add_department),
        )
        .route(
            "/departments/{dep_id}",
            get(routes::departments::get_department)
                .put(routes::departments::update_department)
                .delete(routes::departments::delete_department),
        )
        .route(
            "/departments/{dep_id}/images",
            get(routes::departments::get_department_images)
                .post(routes::departments::add_department_image),
        )
        .route(
            "/departments/{dep_id}/images/{img_id}",
            // get(routes::get_thing_image),
            get(routes::departments::get_department_image)
                .delete(routes::departments::delete_department_image),
        )
        .route(
            "/labels",
            get(routes::things::list_labels).post(routes::things::add_label),
        )
}
