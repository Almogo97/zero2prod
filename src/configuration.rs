#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;
    config.try_deserialize()
}
