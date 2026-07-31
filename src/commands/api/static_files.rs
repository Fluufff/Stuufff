use super::ApiState;
use super::v1::auth::AuthInfo;
use axum::extract::State;
use axum::http::Uri;
use axum::response::{IntoResponse, Response};
use http::{HeaderMap, StatusCode, header};
use mime_guess::mime;
use rust_embed::Embed;
use tracing::debug;

#[derive(Embed)]
#[folder = "web/build"]
struct StaticFiles;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        let mime = mime_guess::from_path(&path);
        let static_file = StaticFiles::get(&path);
        let static_file_html = StaticFiles::get(&format!("{path}.html"));

        match (
            static_file,
            static_file_html,
            mime.is_empty(),
            StaticFiles::get("index.html"),
        ) {
            (Some(static_file), _, _, _) => (
                [(header::CONTENT_TYPE, mime.first_or_octet_stream().as_ref())],
                static_file.data,
            )
                .into_response(),
            (_, Some(static_file_html), _, _) => (
                [(header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())],
                static_file_html.data,
            )
                .into_response(),
            (_, _, true, Some(index_file)) => (
                [(header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())],
                index_file.data,
            )
                .into_response(),
            _ => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

pub async fn static_handler(
    path: Uri,
    headers: HeaderMap,
    State(state): State<ApiState>,
) -> Response {
    let mut path = path.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }
    debug!("serving: {path}");
    match AuthInfo::get_or_redirect(&headers, &state.config, Some(format!("/{path}"))) {
        Ok(_) => StaticFile(path.to_owned()).into_response(),
        Err(r) => r,
    }
}
