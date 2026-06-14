use axum::http::Uri;
use axum::response::{IntoResponse, Response};
use http::{StatusCode, header};
use mime_guess::{Mime, mime};
use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "web/build"]
struct StaticFiles;

pub struct StaticFile<T>(pub T);

fn file(path: &str) -> Option<(EmbeddedFile, Mime)> {
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    file_with_mime(path, mime)
}
fn file_with_mime(path: &str, mime: Mime) -> Option<(EmbeddedFile, Mime)> {
    StaticFiles::get(&path).map(|f| (f, mime))
}

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        let static_file =
            file(&path).or_else(|| file_with_mime(&format!("{}.html", &path), mime::TEXT_HTML));

        match static_file {
            Some((file, mime)) => {
                ([(header::CONTENT_TYPE, mime.as_ref())], file.data).into_response()
            }
            None => match (
                path.split(&['/', '|', ':'])
                    .last()
                    .unwrap_or_default()
                    .contains('.'),
                file("index.html"),
            ) {
                (false, Some((file, mime))) => {
                    ([(header::CONTENT_TYPE, mime.as_ref())], file.data).into_response()
                }
                _ => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            },
        }
    }
}

pub async fn static_handler(path: Uri) -> impl IntoResponse {
    let mut path = path.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }
    StaticFile(path.to_owned())
}
