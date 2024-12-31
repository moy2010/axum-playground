use axum_playground::{app, services::ServicesBuilder};

#[tokio::main]
async fn main() {

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let services = ServicesBuilder {
        ..Default::default()
    }.build();

    axum::serve(listener, app(services)).await.unwrap();
}