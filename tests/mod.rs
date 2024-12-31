use axum::{body::Body, http::Request};
use axum_playground::app;
use tokio::net::TcpListener;
use http_body_util::BodyExt;

#[tokio::test]
async fn the_real_deal() {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
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
    assert_eq!(&body[..], b"Hello, World!");
}