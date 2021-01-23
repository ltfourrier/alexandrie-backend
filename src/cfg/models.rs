use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfiguration {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfiguration {
    pub listen_address: String,
    pub listen_port: u32,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfiguration {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub log: LogConfiguration,
    pub server: ServerConfiguration,
    pub database: DatabaseConfiguration,
}
