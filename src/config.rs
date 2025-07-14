use anyhow::Context;

pub struct Config {
    pub database_url: String,
    pub log_level: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse::<u16>()
                .context("SERVER_PORT must be numeric")?,
        })
    }
}
