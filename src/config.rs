use anyhow::{anyhow, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: MongoDbConfig,
    pub cors: CorsConfig,
    pub telegram: TelegramConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoDbConfig {
    pub uri: String,
    pub database: String,
    pub connection_timeout_ms: u64,
    pub max_pool_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub supports_credentials: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramConfig {
    pub token: Option<String>,
    pub chat_id: Option<String>,
    pub opp_thread_id: Option<u64>,
    pub error_thread_id: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8081,
                log_level: "info".to_string(),
                api_key: None,
            },
            database: MongoDbConfig {
                uri: "mongodb://localhost:27017".to_string(),
                database: "arbitrage_bot".to_string(),
                connection_timeout_ms: 5000,
                max_pool_size: Some(10),
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:3000".to_string()],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                allowed_headers: vec![
                    "Authorization".to_string(),
                    "X-API-Key".to_string(),
                    "Accept".to_string(),
                    "Content-Type".to_string(),
                ],
                supports_credentials: true,
            },
            telegram: TelegramConfig {
                token: None,
                chat_id: None,
                opp_thread_id: None,
                error_thread_id: None,
            },
        }
    }
}

impl MongoDbConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.uri.is_empty() {
            return Err(anyhow!("MongoDB URI not configured"));
        }

        if self.database.is_empty() {
            return Err(anyhow!("MongoDB database name not configured"));
        }

        Ok(())
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Try to load from config directory
        info!("Loading config from file");
        match Self::load_from_file("config/config.toml") {
            Ok(config) => {
                info!("Config loaded from file");
                Ok(config)
            }
            Err(e) => {
                error!("Failed to load config from file: {}", e);
                // Fall back to environment variables or defaults
                info!("Falling back to environment variables or defaults");
                Ok(Self::from_env())
            }
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn from_env() -> Self {
        let mut config = Config::default();

        // Override with environment variables if present
        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.server.host = host;
        }

        if let Ok(port) = std::env::var("SERVER_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.server.port = port_num;
            }
        }

        if let Ok(log_level) = std::env::var("RUST_LOG") {
            config.server.log_level = log_level;
        }

        if let Ok(uri) = std::env::var("MONGODB_URI") {
            config.database.uri = uri;
        }

        if let Ok(db_name) = std::env::var("MONGODB_DATABASE") {
            config.database.database = db_name;
        }

        if let Ok(origins) = std::env::var("CORS_ORIGINS") {
            config.cors.allowed_origins =
                origins.split(',').map(|s| s.trim().to_string()).collect();
        }

        // Telegram configuration
        if let Ok(token) = std::env::var("TELEGRAM_BOT_TOKEN") {
            config.telegram.token = Some(token);
        }

        if let Ok(chat_id) = std::env::var("TELEGRAM_CHAT_ID") {
            config.telegram.chat_id = Some(chat_id);
        }

        if let Ok(api_key) = std::env::var("API_KEY") {
            config.server.api_key = Some(api_key);
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8081);
        assert_eq!(config.database.uri, "mongodb://localhost:27017");
        assert_eq!(config.database.database, "arbitrage_bot");
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("SERVER_PORT", "9090");
        std::env::set_var("MONGODB_URI", "mongodb://test:27017");

        let config = Config::from_env();
        assert_eq!(config.server.port, 9090);
        assert_eq!(config.database.uri, "mongodb://test:27017");

        // Clean up
        std::env::remove_var("SERVER_PORT");
        std::env::remove_var("MONGODB_URI");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_string = toml::to_string_pretty(&config).unwrap();
        assert!(toml_string.contains("127.0.0.1"));
        assert!(toml_string.contains("8081"));
        assert!(toml_string.contains("mongodb://localhost:27017"));
    }
}
