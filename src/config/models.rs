use figment::{
    providers::{Env, Format, Toml},
    Figment, Profile,
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub environment_type: EnvironmentType,
    pub database_config: DatabaseConfig,
    pub server_config: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        let app_config: AppConfig = Figment::new()
            .merge(Toml::file("./src/config/base.toml"))
            .merge(Toml::file("./src/config/development.toml").profile("Development"))
            .merge(Toml::file("./src/config/production.toml").profile("Production"))
            .merge(
                Env::prefixed("DEMO_PRODUCTION__")
                    .split("__")
                    .profile("Production"),
            )
            .select(Profile::from_env_or("environment_type", "Development"))
            .extract()
            .expect("Error when deserializing AppConfig");

        app_config
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum EnvironmentType {
    Development,
    Production,
}

#[derive(Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub exporter: Option<LogExporter>,
}

#[derive(Deserialize, Clone)]
pub struct LogExporter {
    pub otel_api_key: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub address: String,
}
