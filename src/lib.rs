pub mod services;

use axum::{
    extract::State, routing::{get, post}, Json, Router
};
use services::Services;

async fn handler(State(services): State<Services>) -> String {
    services.simple.get().await
}

pub fn app(services: Services) -> Router {
    Router::new()
        .route("/", get(handler))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
        .with_state(services)
}