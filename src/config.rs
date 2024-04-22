use std::env;
use dotenv::dotenv;

pub struct Config {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub node_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Load environment variables from .env file
        dotenv().ok();
        let domain = env::var("DOMAIN").map_err(|_| "Missing DOMAIN environment variable")?;
        let username = env::var("USERNAME").map_err(|_| "Missing USERNAME environment variable")?;
        let password = env::var("PASSWORD").map_err(|_| "Missing PASSWORD environment variable")?;
        let node_id = env::var("NODE_ID").map_err(|_| "Missing NODE_ID environment variable")?;

        Ok(Config {
            domain,
            username,
            password,
            node_id,
        })
    }
}