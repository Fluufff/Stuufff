use std::path::PathBuf;

use tokio::select;
use tracing::info;

use crate::{cli::DBArgs, config::error::RuntimeError};
use config::ParsedConfig;

mod config;
use axum::{Router, response::IntoResponse};
use http::{StatusCode, Uri, header};
use mime_guess::mime;
use static_files::static_handler;
use std::net::{Ipv6Addr, SocketAddrV6};

mod static_files;

fn api_router(_v1state: ApiState) -> Router {
    Router::new()
        // .nest("/v0beta1", v1::init().with_state(v1state.clone()))
        .fallback(fallback)
}

#[derive(Clone)]
pub struct ApiState {
    pub config: ParsedConfig,
}

impl ApiState {
    pub async fn new(config: ParsedConfig) -> Result<Self, RuntimeError> {
        Ok(Self { config })
    }
}

pub async fn fallback(path: Uri) -> impl IntoResponse {
    ((
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())],
        format!(
            r#"{{"error": "404 not found", "description": "no endpoint at path /api{}"}}"#,
            path
        ),
    ),)
        .into_response()
}

#[derive(clap::Args)]
pub struct RunInput {
    #[arg(short, long, value_name = "PATH")]
    config_file: Option<PathBuf>,
    #[command(flatten)]
    db_args: DBArgs,
}
pub async fn run(input: RunInput) -> Result<(), RuntimeError> {
    let config: ParsedConfig = config::Config::from_input(input).await?.parse().await?;

    info!(database = config.database.host.get()?, "starting");

    let state = ApiState::new(config.clone()).await?;
    let router = Router::new()
        // .nest("/z", z::init().with_state(state.clone()))
        .nest("/api", api_router(state))
        // .nest("/swagger", swagger::router())
        .fallback(static_handler);
    // .layer(
    //     ServiceBuilder::new()
    //         .layer(HandleErrorLayer::new(|err: BoxError| async move {
    //             (
    //                 StatusCode::INTERNAL_SERVER_ERROR,
    //                 format!("Unhandled error: {}", err),
    //             )
    //         }))
    //         .layer(CorsLayer::new().allow_origin(cors::Any).allow_methods([
    //             http::Method::GET,
    //             http::Method::POST,
    //             http::Method::PUT,
    //         ])),
    // );

    let addr = SocketAddrV6::new(Ipv6Addr::from_bits(0), 3000, 0, 0);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);

    // axum::serve is Infallible (even though it's return type is (). No point dealing with the handle
    let api_handle = tokio::spawn(axum::serve(listener, router.into_make_service()).into_future());

    let e = select! {
        _ = api_handle => "api listener is not supposed to stop",
        // _ = caches.run(&config) => "cache runner is not supposed to stop",
    };
    Err(RuntimeError::JobStopped(e))
}
