use async_trait::async_trait;

#[async_trait]
pub trait SimpleService {
    async fn get(&self) -> String;
}

#[derive(Clone)]
pub struct SimpleServiceImpl;

#[async_trait]
impl SimpleService for SimpleServiceImpl {
    async fn get(&self) -> String {
        "Hello dependency injection".to_owned()
    }
}