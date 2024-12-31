use std::sync::Arc;

use async_trait::async_trait;
use axum::{body::Body, http::Request};
use axum_playground::{app, services::{simple::SimpleService, ServicesBuilder}};
use tokio::net::TcpListener;
use http_body_util::BodyExt;

struct SimpleServiceMock;

#[async_trait]
impl SimpleService for SimpleServiceMock {
    async fn get(&self) -> String {
        "Mocked from test".to_owned()
    }
}

#[tokio::test]
async fn the_real_deal() {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let services = ServicesBuilder {
        simple: Some(Arc::new(SimpleServiceMock)),
        ..Default::default()
    }.build();

    tokio::spawn(async move {
        axum::serve(listener, app(services)).await.unwrap();
    });

    let client =
        hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
            .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Mocked from test");
}