use tokio::select;
use tokio::signal::unix::{SignalKind, signal};
use tracing::{error, info, level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    init_logging();

    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| "Failed to install rustls crypto provider")?;

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    select! {
        result = logistics_inventory::run() => {
            if let Err(err) = result {
                error!("error: {}", err);
                return Err("exiting due to failure");
            }
        },
        _ = sigterm.recv() => warn!("SIGTERM received"),
        _ = sigint.recv() => warn!("SIGINT received"),
    }
    info!("exiting");
    Ok(())
}

fn init_logging() {
    let f = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .expect("invalid RUST_LOG directives")
        .add_directive(
            "kube_client::client::tls::rustls_tls=error"
                .parse()
                .unwrap(),
        )
        .add_directive("tower::buffer::worker=error".parse().unwrap())
        .add_directive(
            "hyper_util::client::legacy::connect::http=error"
                .parse()
                .unwrap(),
        )
        .add_directive("rustls::client::hs=error".parse().unwrap())
        .add_directive("rustls::client::tls13=error".parse().unwrap())
        .add_directive("rustls::client::common=error".parse().unwrap())
        .add_directive("hyper_util::client::legacy::pool=error".parse().unwrap())
        .add_directive("hyper_rustls::config=error".parse().unwrap())
        .add_directive("rustls::common_state=error".parse().unwrap())
        .add_directive("kube_client::client::builder=error".parse().unwrap());
    tracing_subscriber::registry()
        .with(f)
        .with(tracing_subscriber::fmt::layer())
        // .with(ErrorLayer::default())
        .init();
}
