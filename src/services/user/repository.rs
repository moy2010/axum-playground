use std::sync::Arc;

use async_trait::async_trait;
use secrecy::ExposeSecret;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::error::DemoError;

use super::models::{User, UserId, UserRaw, UserUpdate};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> Result<User, DemoError>;

    async fn get_by_id(&self, user_id: UserId) -> Result<User, DemoError>;

    async fn update(&self, user_id: UserId, updates: Vec<UserUpdate>) -> Result<User, DemoError>;

    async fn delete(&self, user_id: UserId) -> Result<(), DemoError>;
}

#[derive(Clone)]
pub struct UserRepositorySqLite {
    pool: Arc<SqlitePool>,
}

impl UserRepositorySqLite {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySqLite {
    async fn create_user(&self, user: User) -> Result<User, DemoError> {
        let mut conn = self.pool.acquire().await?;

        let id = user.id.as_ref().to_owned();
        let name = user.name.as_ref().to_owned();
        let exposed_email_address = user.email_address.expose_secret().as_ref().to_owned();

        sqlx::query!(
            r#"
    INSERT INTO users ( id, name, email_address, created_at )
    VALUES ( ?1, ?2, ?3, ?4 )
            "#,
            id,
            name,
            exposed_email_address,
            user.created_at
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(user)
    }

    async fn get_by_id(&self, user_id: UserId) -> Result<User, DemoError> {
        sqlx::query_as::<_, UserRaw>(
            "SELECT id, name, email_address, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(user_id.as_ref())
        .fetch_optional(&*self.pool)
        .await?
        .map(|user_raw| User::try_from(user_raw))
        .ok_or(DemoError::ResourceNotFound)?
    }

    async fn update(&self, user_id: UserId, updates: Vec<UserUpdate>) -> Result<User, DemoError> {
        if updates.is_empty() {
            return Err(DemoError::Validation {
                message: "List of updates was empty".to_owned(),
            });
        }

        let mut query =
            QueryBuilder::<Sqlite>::new("UPDATE users SET updated_at = datetime('now')");

        for update in updates {
            match update {
                UserUpdate::Name { value } => {
                    query.push(", name = ");
                    query.push_bind(value.as_ref().to_owned());
                }
                UserUpdate::EmailAddress { value } => {
                    query.push(", email_address = ");
                    query.push_bind(value.expose_secret().as_ref().to_owned());
                }
            }
        }

        query.push(" WHERE id = ");
        query.push_bind(user_id.as_ref().to_owned());
        query.push(" RETURNING *");

        query
            .build_query_as::<UserRaw>()
            .fetch_one(&*self.pool)
            .await
            .map(|user_raw| User::try_from(user_raw))?
    }

    async fn delete(&self, user_id: UserId) -> Result<(), DemoError> {
        let id = user_id.as_ref().to_owned();

        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(())
    }
}
