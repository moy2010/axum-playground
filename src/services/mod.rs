use std::sync::Arc;

use simple::{SimpleService, SimpleServiceImpl};

pub mod simple;

#[derive(Clone)]
pub struct Services {
    pub simple: Arc<dyn SimpleService + Send + Sync>
}

#[derive(Default)]
pub struct ServicesBuilder {
    pub simple: Option<Arc<dyn SimpleService + Send + Sync>>
}

impl ServicesBuilder {
    pub fn build(self) -> Services {
        Services {
            simple: self.simple.unwrap_or(Arc::new(SimpleServiceImpl)) 
        }
    }
}