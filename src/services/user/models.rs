use chrono::{DateTime, Utc};
use secrecy::{zeroize::Zeroize, CloneableSecret, SecretBox, SerializableSecret};
use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;
use sqlx::FromRow;
use std::str::FromStr;
use tracing::debug;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::error::DemoError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserId(pub Uuid);

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Clone, Debug, DeserializeFromStr, Serialize)]
pub struct UserName(String);

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for UserName {
    fn default() -> Self {
        "John Doe"
            .parse()
            .expect("Error creating default user name")
    }
}

const MAX_USERNAME_LENGTH: usize = 100;
const MAX_EMAIL_ADDRESS_LENGTH: usize = 100;

impl FromStr for UserName {
    type Err = DemoError;

    fn from_str(value: &str) -> Result<UserName, Self::Err> {
        let trimmed_value = value.trim();

        let is_empty_or_whitespace = trimmed_value.is_empty();

        let is_too_long = value.graphemes(true).count() > MAX_USERNAME_LENGTH;

        let error_message: String;

        if is_empty_or_whitespace {
            error_message = "User name cannot be empty".to_owned();
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else if is_too_long {
            error_message =
                format!("User name is too long. Maximum valid length is {MAX_USERNAME_LENGTH}");
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else {
            Ok(Self(trimmed_value.to_string()))
        }
    }
}

#[derive(Clone, Serialize, Debug, Eq, PartialEq, DeserializeFromStr)]
pub struct EmailAddress(String);

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for EmailAddress {
    fn default() -> Self {
        "me@mail.com"
            .parse()
            .expect("Error creating default email address")
    }
}

/// Permits cloning the secret
impl CloneableSecret for EmailAddress {}

impl Zeroize for EmailAddress {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl SerializableSecret for EmailAddress {}

pub type SecretEmailAddress = SecretBox<EmailAddress>;

impl FromStr for EmailAddress {
    type Err = DemoError;

    fn from_str(value: &str) -> Result<EmailAddress, Self::Err> {
        let trimmed_value = value.trim();

        let is_empty_or_whitespace = trimmed_value.is_empty();

        let is_too_long = trimmed_value.graphemes(true).count() > MAX_EMAIL_ADDRESS_LENGTH;

        // Iterate over all characters in the input `value` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = trimmed_value
            .chars()
            .any(|g| forbidden_characters.contains(&g));

        let contains_at_symbol = trimmed_value.chars().find(|char| *char == '@').is_some();

        let error_message: String;

        if !contains_at_symbol {
            error_message = "Email address must have the @ symbol".to_owned();
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else if is_empty_or_whitespace {
            error_message = "Email address cannot be empty".to_owned();
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else if is_too_long {
            error_message = format!(
                "Email address is too long. Maximum valid length is {MAX_EMAIL_ADDRESS_LENGTH}"
            );
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else if contains_forbidden_characters {
            error_message = format!(
                "Email address is not valid. It should not contain any of the following characters: {}", forbidden_characters.map(|char| char.to_string()).join(", ")
            );
            debug!(error_message);
            Err(DemoError::Validation {
                message: error_message,
            })
        } else {
            Ok(Self(trimmed_value.to_string()))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email_address: SecretEmailAddress,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserRaw> for User {
    type Error = DemoError;

    fn try_from(user_raw: UserRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            id: UserId(user_raw.id),
            name: user_raw.name.parse()?,
            email_address: SecretBox::new(Box::new(user_raw.email_address.parse()?)),
            created_at: user_raw.created_at,
            updated_at: user_raw.updated_at,
        })
    }
}

#[derive(Debug, Default, Serialize, FromRow)]
pub struct UserRaw {
    pub id: Uuid,
    pub name: String,
    pub email_address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub name: UserName,
    pub email_address: SecretEmailAddress,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub updates: Vec<UserUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum UserUpdate {
    Name { value: UserName },
    EmailAddress { value: SecretEmailAddress },
}
