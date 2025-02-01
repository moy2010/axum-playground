use std::sync::Arc;

use sqlx::SqlitePool;
use user::{
    repository::UserRepositorySqLite,
    service::{UserService, UserServiceImpl},
};

use crate::config::models::AppConfig;

pub mod user;

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<dyn UserService + Send + Sync>,
}

#[derive(Default)]
pub struct ServicesBuilder {
    pub user_service: Option<Arc<dyn UserService + Send + Sync>>,
}

impl ServicesBuilder {
    pub async fn build(self, app_config: AppConfig) -> Services {
        let pool = SqlitePool::connect(&app_config.database_config.address)
            .await
            .expect("Error creating SQlite pool");

        let pool = Arc::new(pool);

        Services {
            user_service: self
                .user_service
                .unwrap_or(Arc::new(UserServiceImpl::new(Box::new(
                    UserRepositorySqLite::new(pool.clone()),
                )))),
        }
    }
}
