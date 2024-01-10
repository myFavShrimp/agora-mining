use std::net::SocketAddr;

use serde::Deserialize;
use serde_env::from_env;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    host_name: String,
    port: u16,
    pub database_url: String,
}

impl Config {
    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.host_name, self.port)
            .parse()
            .expect("valid host address")
    }

    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        from_env::<Config>().expect("load config")
    }
}
