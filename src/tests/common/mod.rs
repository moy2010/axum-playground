use crate::{
    api::app,
    config::models::AppConfig,
    services::{Services, ServicesBuilder},
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Default)]
pub struct TestAppBuilder {
    pub services_builder: Option<ServicesBuilder>,
}

pub struct TestApp {
    pub address: String,
    pub services: Arc<Services>,
}

impl TestAppBuilder {
    pub async fn build(self) -> TestApp {
        let test_app_config = AppConfig::load();
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let services = Arc::new(
            self.services_builder
                .unwrap_or(ServicesBuilder {
                    ..Default::default()
                })
                .build(test_app_config)
                .await,
        );

        let services_clone = Arc::clone(&services);

        tokio::spawn(async move {
            axum::serve(listener, app(services_clone)).await.unwrap();
        });

        TestApp {
            address: address.to_string(),
            services,
        }
    }
}
