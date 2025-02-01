use std::{error::Error, future::ready, sync::Arc};

use crate::{
    error::DemoError,
    services::{
        user::{
            models::{User, UserId},
            repository::MockUserRepository,
            service::UserServiceImpl,
        },
        ServicesBuilder,
    },
    tests::common::TestAppBuilder,
};
use secrecy::ExposeSecret;
use serde_json::json;

#[tokio::test]
async fn users_can_be_created_successfully() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let response = reqwest::Client::new()
        .post(&format!("http://{}/users", &test_app.address))
        .json(&json!({"name": "Jon Jonsson", "email_address": "some@email.com"}))
        .send()
        .await
        .unwrap();

    let response_status = response.status();

    let user: User = response.json().await.unwrap();

    assert_eq!(201, response_status.as_u16());
    assert_eq!("Jon Jonsson", user.name.as_ref());
    assert_eq!(
        "some@email.com",
        user.email_address.expose_secret().as_ref()
    );
}

#[tokio::test]
async fn users_cannot_be_created_when_an_invalid_email_address_is_provided() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let response = reqwest::Client::new()
        .post(&format!("http://{}/users", &test_app.address))
        .json(&json!({"name": "Jon Jonsson", "email_address": "not an email address"}))
        .send()
        .await
        .unwrap();

    let response_status = response.status();

    assert_eq!(422, response_status.as_u16());
}

#[tokio::test]
async fn users_cannot_be_created_when_an_email_address_with_invalid_characters_is_provided() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let response = reqwest::Client::new()
        .post(&format!("http://{}/users", &test_app.address))
        .json(&json!({"name": "Jon Jonsson", "email_address": "invalid_email/@domain.com"}))
        .send()
        .await
        .unwrap();

    let response_status = response.status();

    assert_eq!(422, response_status.as_u16());
}

#[tokio::test]
async fn users_cannot_be_created_when_a_user_name_too_long_is_provided() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let response = reqwest::Client::new()
        .post(&format!("http://{}/users", &test_app.address))
        .json(&json!({"name": "Oh long Jonsson this is an incredibly long name that should never be accepted as valid as long as there's fresh water on planet earth", "email_address": "email@domain.com"}))
        .send()
        .await
        .unwrap();

    let response_status = response.status();

    assert_eq!(422, response_status.as_u16());
}

#[tokio::test]
async fn a_users_name_can_be_updated() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let user = User {
        ..Default::default()
    };

    test_app
        .services
        .user_service
        .create(user.clone())
        .await
        .unwrap();

    // We update the user name
    let update_user_name_response = reqwest::Client::new()
        .patch(&format!(
            "http://{}/users/{}",
            &test_app.address,
            user.id.as_ref()
        ))
        .json(&json!({"updates": [{"type": "Name", "value": "Totally new name"}] }))
        .send()
        .await
        .unwrap();

    let response_status = update_user_name_response.status();

    assert_eq!(200, response_status.as_u16());

    let updated_user: User = update_user_name_response.json().await.unwrap();

    assert_eq!("Totally new name", updated_user.name.as_ref());
}

#[tokio::test]
async fn a_users_email_address_can_be_updated() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let user = User {
        ..Default::default()
    };

    test_app
        .services
        .user_service
        .create(user.clone())
        .await
        .unwrap();

    // We update the email address
    let update_user_name_response = reqwest::Client::new()
        .patch(&format!(
            "http://{}/users/{}",
            &test_app.address,
            user.id.as_ref()
        ))
        .json(&json!({"updates": [{"type": "EmailAddress", "value": "brand_new_email@address"}] }))
        .send()
        .await
        .unwrap();

    let response_status = update_user_name_response.status();

    assert_eq!(200, response_status.as_u16());

    let updated_user: User = update_user_name_response.json().await.unwrap();

    assert_eq!(
        "brand_new_email@address",
        updated_user.email_address.expose_secret().as_ref()
    );
}

#[tokio::test]
async fn users_can_be_fetched() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let user = User {
        ..Default::default()
    };

    test_app
        .services
        .user_service
        .create(user.clone())
        .await
        .unwrap();

    // We fetch the user
    let fetch_user_response = reqwest::Client::new()
        .get(&format!(
            "http://{}/users/{}",
            &test_app.address,
            user.id.as_ref()
        ))
        .send()
        .await
        .unwrap();

    let response_status = fetch_user_response.status();

    assert_eq!(200, response_status.as_u16());

    // We assert that the response can be deserialized as a User
    assert!(fetch_user_response.json::<User>().await.is_ok())
}

#[tokio::test]
async fn users_can_be_deleted() {
    let test_app = TestAppBuilder {
        ..Default::default()
    }
    .build()
    .await;

    let user = User {
        ..Default::default()
    };

    test_app
        .services
        .user_service
        .create(user.clone())
        .await
        .unwrap();

    // We delete the user
    let delete_user_response = reqwest::Client::new()
        .delete(&format!(
            "http://{}/users/{}",
            &test_app.address,
            user.id.as_ref()
        ))
        .send()
        .await
        .unwrap();

    let delete_user_response_status = delete_user_response.status();

    assert_eq!(200, delete_user_response_status.as_u16());

    // We attempt to fetch the user again after being deleted
    let fetch_user_response = reqwest::Client::new()
        .get(&format!(
            "http://{}/users/{}",
            &test_app.address,
            user.id.as_ref()
        ))
        .send()
        .await
        .unwrap();

    let fetch_user_response_status = fetch_user_response.status();

    assert_eq!(404, fetch_user_response_status.as_u16());
}

#[tokio::test]
async fn we_return_500_if_repository_returns_an_io_error_when_attempting_to_delete_a_user() {
    let mut mock = MockUserRepository::new();

    mock.expect_delete().returning(|_| {
        Box::pin(ready(Err(DemoError::IO {
            cause: Box::<dyn Error + Send + Sync>::from("Fake IO Error"),
        })))
    });

    let test_app = TestAppBuilder {
        services_builder: Some(ServicesBuilder {
            user_service: Some(Arc::new(UserServiceImpl::new(Box::new(mock)))),
            ..Default::default()
        }),
        ..Default::default()
    }
    .build()
    .await;

    // We delete the user
    let delete_user_response = reqwest::Client::new()
        .delete(&format!(
            "http://{}/users/{}",
            &test_app.address,
            UserId {
                ..Default::default()
            }
            .as_ref()
        ))
        .send()
        .await
        .unwrap();

    let delete_user_response_status = delete_user_response.status();

    assert_eq!(500, delete_user_response_status.as_u16());
}
