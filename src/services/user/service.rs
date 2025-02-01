use async_trait::async_trait;

use crate::error::DemoError;

use super::{
    models::{User, UserId, UserUpdate},
    repository::UserRepository,
};

#[async_trait]
pub trait UserService {
    async fn create(&self, user: User) -> Result<User, DemoError>;

    async fn get_by_id(&self, user_id: UserId) -> Result<User, DemoError>;

    async fn update(&self, user_id: UserId, updates: Vec<UserUpdate>) -> Result<User, DemoError>;

    async fn delete(&self, user_id: UserId) -> Result<(), DemoError>;
}

pub struct UserServiceImpl {
    repository: Box<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub fn new(repository: Box<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: User) -> Result<User, DemoError> {
        self.repository.create_user(user).await
    }

    async fn get_by_id(&self, user_id: UserId) -> Result<User, DemoError> {
        self.repository.get_by_id(user_id).await
    }

    async fn update(&self, user_id: UserId, updates: Vec<UserUpdate>) -> Result<User, DemoError> {
        self.repository.update(user_id, updates).await
    }

    async fn delete(&self, user_id: UserId) -> Result<(), DemoError> {
        self.repository.delete(user_id).await
    }
}
