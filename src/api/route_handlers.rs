use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;

use crate::{
    error::DemoError,
    services::{
        user::models::{CreateUserPayload, UpdateUserPayload, User, UserId},
        Services,
    },
};

pub async fn create_user(
    State(services): State<Arc<Services>>,
    payload: Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), DemoError> {
    let user = User {
        name: payload.0.name,
        email_address: payload.0.email_address,
        created_at: Utc::now(),
        ..Default::default()
    };
    services
        .user_service
        .create(user)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
}

pub async fn get_user_by_id(
    State(services): State<Arc<Services>>,
    user_id: Path<UserId>,
) -> Result<Json<User>, DemoError> {
    services.user_service.get_by_id(user_id.0).await.map(Json)
}

pub async fn update_user(
    State(services): State<Arc<Services>>,
    user_id: Path<UserId>,
    payload: Json<UpdateUserPayload>,
) -> Result<Json<User>, DemoError> {
    services
        .user_service
        .update(user_id.0, payload.0.updates)
        .await
        .map(Json)
}

pub async fn delete_user(
    State(services): State<Arc<Services>>,
    user_id: Path<UserId>,
) -> Result<(), DemoError> {
    services.user_service.delete(user_id.0).await
}
