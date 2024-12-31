use axum::{
    routing::{get, post},
    Json, Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
}