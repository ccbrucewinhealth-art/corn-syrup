use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use sea_orm::{ConnectOptions, DatabaseConnection};

use crate::backend::database::DatabaseDialect;
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrmFramework {
    SeaOrm,
}

#[derive(Debug, Clone)]
pub struct OrmConfig {
    pub framework: OrmFramework,
    pub dialect: DatabaseDialect,
    pub url: String,
    pub database: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub sqlx_logging: bool,
}

#[derive(Debug, Clone)]
pub struct OrmRuntime {
    pub config: OrmConfig,
    pub connected: bool,
}

impl OrmFramework {
    pub fn as_str(&self) -> &'static str {
        logging::debug("database.orm", "framework.as_str", "enter");
        match self {
            OrmFramework::SeaOrm => "SeaORM",
        }
    }
}

impl OrmConfig {
    pub fn from_env_and_sqlite_path(env: &HashMap<String, String>, sqlite_path: &Path) -> Self {
        logging::debug("database.orm", "from_env_and_sqlite_path", "enter");
        let dialect = parse_database_dialect(env.get("DATABASE_TYPE").map(String::as_str));
        let host = env_value(env, "DATABASE_HOST");
        let database = env_value(env, "DATABASE_NAME");
        let username = env_value(env, "DATABASE_USER");
        let password = env_value(env, "DATABASE_PASSWORD");
        let port = env
            .get("DATABASE_PORT")
            .and_then(|value| value.parse::<u16>().ok())
            .or_else(|| default_port(&dialect));
        let url = env
            .get("DATABASE_URL")
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| build_database_url(&dialect, sqlite_path, host.as_deref(), port, database.as_deref(), username.as_deref(), password.as_deref()));

        Self {
            framework: OrmFramework::SeaOrm,
            dialect,
            url,
            database,
            host,
            port,
            username,
            max_connections: env_u32(env, "DATABASE_MAX_CONNECTIONS", 10),
            min_connections: env_u32(env, "DATABASE_MIN_CONNECTIONS", 1),
            connect_timeout_seconds: env_u64(env, "DATABASE_CONNECT_TIMEOUT_SECONDS", 8),
            sqlx_logging: env_bool(env, "DATABASE_SQLX_LOGGING", false),
        }
    }

    pub fn connect_options(&self) -> ConnectOptions {
        logging::debug("database.orm", "connect_options", "enter");
        let mut options = ConnectOptions::new(self.url.clone());
        options
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .connect_timeout(Duration::from_secs(self.connect_timeout_seconds))
            .sqlx_logging(self.sqlx_logging);
        options
    }

    pub fn redacted_url(&self) -> String {
        logging::debug("database.orm", "redacted_url", "enter");
        redact_database_url(&self.url)
    }

    pub fn as_pairs(&self) -> Vec<(String, String)> {
        logging::debug("database.orm", "as_pairs", "enter");
        vec![
            ("ormFramework".to_string(), self.framework.as_str().to_string()),
            ("dialect".to_string(), self.dialect.as_str().to_string()),
            ("url".to_string(), self.redacted_url()),
            ("database".to_string(), self.database.clone().unwrap_or_default()),
            ("host".to_string(), self.host.clone().unwrap_or_default()),
            ("port".to_string(), self.port.map(|value| value.to_string()).unwrap_or_default()),
            ("username".to_string(), self.username.clone().unwrap_or_default()),
            ("maxConnections".to_string(), self.max_connections.to_string()),
            ("minConnections".to_string(), self.min_connections.to_string()),
            ("connectTimeoutSeconds".to_string(), self.connect_timeout_seconds.to_string()),
        ]
    }
}

impl OrmRuntime {
    pub fn new(config: OrmConfig) -> Self {
        logging::debug("database.orm", "runtime.new", "enter");
        Self {
            config,
            connected: false,
        }
    }

    pub async fn connect(&mut self) -> Result<DatabaseConnection, String> {
        logging::debug("database.orm", "runtime.connect", self.config.redacted_url());
        let connection = sea_orm::Database::connect(self.config.connect_options())
            .await
            .map_err(|err| format!("SeaORM connect failed: {err}"))?;
        self.connected = true;
        Ok(connection)
    }
}

pub fn parse_database_dialect(value: Option<&str>) -> DatabaseDialect {
    logging::debug("database.orm", "parse_database_dialect", "enter");
    match value.unwrap_or("sqlite").trim().to_ascii_lowercase().as_str() {
        "mariadb" => DatabaseDialect::MariaDb,
        "mysql" => DatabaseDialect::Mysql,
        _ => DatabaseDialect::Sqlite,
    }
}

fn env_value(env: &HashMap<String, String>, key: &str) -> Option<String> {
    env.get(key)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_u32(env: &HashMap<String, String>, key: &str, default_value: u32) -> u32 {
    env.get(key)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(default_value)
}

fn env_u64(env: &HashMap<String, String>, key: &str, default_value: u64) -> u64 {
    env.get(key)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default_value)
}

fn env_bool(env: &HashMap<String, String>, key: &str, default_value: bool) -> bool {
    env.get(key)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON"))
        .unwrap_or(default_value)
}

fn default_port(dialect: &DatabaseDialect) -> Option<u16> {
    match dialect {
        DatabaseDialect::Sqlite => None,
        DatabaseDialect::MariaDb | DatabaseDialect::Mysql => Some(3306),
    }
}

fn build_database_url(
    dialect: &DatabaseDialect,
    sqlite_path: &Path,
    host: Option<&str>,
    port: Option<u16>,
    database: Option<&str>,
    username: Option<&str>,
    password: Option<&str>,
) -> String {
    logging::debug("database.orm", "build_database_url", "enter");
    match dialect {
        DatabaseDialect::Sqlite => format!("sqlite://{}?mode=rwc", sqlite_path.display()),
        DatabaseDialect::MariaDb | DatabaseDialect::Mysql => format!(
            "mysql://{}:{}@{}:{}/{}",
            username.unwrap_or("root"),
            password.unwrap_or_default(),
            host.unwrap_or("127.0.0.1"),
            port.unwrap_or(3306),
            database.unwrap_or("corn_syrup")
        ),
    }
}

fn redact_database_url(url: &str) -> String {
    if let Some((scheme, rest)) = url.split_once("://") {
        if let Some((credentials, location)) = rest.split_once('@') {
            if let Some((username, _)) = credentials.split_once(':') {
                return format!("{scheme}://{username}:***@{location}");
            }
        }
    }
    url.to_string()
}
