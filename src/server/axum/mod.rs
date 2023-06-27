use axum::{Router, Server};
use proc_macros_rs::DisplayUsingDebug;
use serde::{Deserialize, Serialize};
use tokio::signal;
use tracing::info;

pub mod common_routes;

#[derive(Debug, Default, DisplayUsingDebug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Config {
    port: u16,
}

pub async fn start(config: Config, app: Router) {
    let addr = format!("0.0.0.0:{}", config.port);
    info!("starting server on {addr}");

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}
