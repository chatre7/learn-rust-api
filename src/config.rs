use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let database_url = env::var("DATABASE_URL")?;
        let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
        Ok(Self { database_url, port })
    }
}

