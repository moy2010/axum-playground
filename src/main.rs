use std::{env, sync::Arc};

use axum_playground::{api::app, config::models::AppConfig, services::ServicesBuilder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let app_config = AppConfig::load();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=info,tower_http=info", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind(&app_config.server_config.address)
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let services = Arc::new(
        ServicesBuilder {
            ..Default::default()
        }
        .build(app_config)
        .await,
    );

    axum::serve(listener, app(services)).await.unwrap();
}
