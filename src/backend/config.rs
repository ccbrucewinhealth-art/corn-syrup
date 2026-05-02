use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::backend::logging;

const DEFAULT_ENV_CONTENT: &str = "LOG_LEVEL=debug\nCORN_SYRUP_BACKEND_HOST=127.0.0.1\nCORN_SYRUP_BACKEND_PORT=3001\nCORN_SYRUP_BACKEND_CORS_ORIGIN=*\nDATABASE_TYPE=mysql\nDATABASE_HOST=192.168.0.25\nDATABASE_PORT=3306\nDATABASE_NAME=TRC_RData\nDATABASE_USER=TRC201\nDATABASE_PASSWORD=syscom#1\n";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub args: HashMap<String, String>,
    pub hostname: Option<String>,
    pub port: u16,
    pub is_ssl: bool,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
    pub ssl_key_passphrase: Option<String>,
    pub local_websocket_url: String,
    pub demo_mode: bool,
    pub data_dir: PathBuf,
    pub node_env: String,
    pub ws_origin_check: String,
    pub cloudflared_token: Option<String>,
    pub disable_frame_same_origin: bool,
    pub test_mode: bool,
}

pub fn merge_env_with_dotenv(env_dir: impl AsRef<Path>, env: &[(String, String)]) -> HashMap<String, String> {
    logging::debug("auto.config", "merge_env_with_dotenv", "enter");
    let mut merged_env = env_map(&load_dotenv_from(env_dir));
    merged_env.extend(env_map(env));
    merged_env
}

fn env_map(env: &[(String, String)]) -> HashMap<String, String> {
    env.iter().cloned().collect()
}

fn parse_bool_flag(value: Option<&String>) -> bool {
    matches!(
        value.map(|v| v.as_str()),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}

pub fn parse_args(args: &[String]) -> HashMap<String, String> {
    logging::debug("auto.config", "parse_args", "enter");
    let mut parsed = HashMap::new();
    let mut index = 0;

    // 對齊 uptime-kuma server/config.js 使用 args-parser 的慣例：
    // --name=value、--name value 與單純 --flag 都會轉成可查詢的 key。
    while index < args.len() {
        let arg = &args[index];
        if let Some(raw) = arg.strip_prefix("--") {
            if let Some((key, value)) = raw.split_once('=') {
                parsed.insert(key.to_string(), value.to_string());
            } else if args
                .get(index + 1)
                .is_some_and(|next| !next.starts_with('-'))
            {
                parsed.insert(raw.to_string(), args[index + 1].clone());
                index += 1;
            } else {
                parsed.insert(raw.to_string(), "true".to_string());
            }
        }
        index += 1;
    }

    parsed
}

pub fn load_dotenv_from(env_dir: impl AsRef<Path>) -> Vec<(String, String)> {
    logging::debug("auto.config", "load_dotenv_from", "enter");
    let env_path = env_dir.as_ref().join(".env");

    if !env_path.exists() {
        if let Some(parent) = env_path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                logging::debug("auto.config", "load_dotenv_from", format!("create env dir failed: {err}"));
            }
        }
        if let Err(err) = fs::write(&env_path, DEFAULT_ENV_CONTENT) {
            logging::debug("auto.config", "load_dotenv_from", format!("create env file failed: {err}"));
            return Vec::new();
        }
        logging::debug("auto.config", "load_dotenv_from", format!("created default env: {}", env_path.display()));
    }

    logging::debug("auto.config", "load_dotenv_from", format!("using env file: {}", env_path.display()));

    let Ok(content) = fs::read_to_string(env_path) else {
        return Vec::new();
    };

    // dotenv 轉譯採「逐行、忽略註解、第一個 '=' 分割」演算法；
    // 這可涵蓋 uptime-kuma 以 dotenv.config() 載入 key/value 的主要行為。
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            let value = value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string();
            Some((key.trim().to_string(), value))
        })
        .collect()
}

pub fn load_config(args: &[String], env: &[(String, String)]) -> Result<AppConfig, String> {
    logging::debug("auto.config", "load_config", "enter");
    let args_map = parse_args(args);
    let merged_env = merge_env_with_dotenv(".", env);
    let log_level = merged_env
        .get("LOG_LEVEL")
        .or_else(|| merged_env.get("RUST_LOG"))
        .map(String::as_str)
        .unwrap_or("debug");
    logging::init(log_level);
    logging::debug("auto.config", "load_config", format!("log level: {log_level}"));

    let hostname = args_map
        .get("host")
        .cloned()
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_HOST").cloned())
        .or_else(|| merged_env.get("HOST").cloned());

    let port = args_map
        .get("port")
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_PORT"))
        .or_else(|| merged_env.get("PORT"))
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3001);
    logging::debug(
        "auto.config",
        "load_config",
        format!(
            "listen host={}, port={}",
            hostname.as_deref().unwrap_or("0.0.0.0"),
            port
        ),
    );

    let ssl_key = args_map
        .get("ssl-key")
        .cloned()
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_SSL_KEY").cloned())
        .or_else(|| merged_env.get("SSL_KEY").cloned());
    let ssl_cert = args_map
        .get("ssl-cert")
        .cloned()
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_SSL_CERT").cloned())
        .or_else(|| merged_env.get("SSL_CERT").cloned());
    let ssl_key_passphrase = args_map
        .get("ssl-key-passphrase")
        .cloned()
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_SSL_KEY_PASSPHRASE").cloned())
        .or_else(|| merged_env.get("SSL_KEY_PASSPHRASE").cloned());
    let is_ssl = ssl_key.is_some() && ssl_cert.is_some();

    // getLocalWebSocketURL() 對齊 config.js：SSL 決定 wss/ws，hostname 空值改 localhost。
    let host = hostname.clone().unwrap_or_else(|| "localhost".to_string());
    let protocol = if is_ssl { "wss" } else { "ws" };
    let local_websocket_url = format!("{protocol}://{host}:{port}");

    let demo_mode = parse_bool_flag(args_map.get("demo"));
    let data_dir = args_map
        .get("data-dir")
        .or_else(|| merged_env.get("DATA_DIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./data"));

    let node_env = merged_env
        .get("NODE_ENV")
        .cloned()
        .unwrap_or_else(|| "production".to_string());
    let ws_origin_check = merged_env
        .get("CORN_SYRUP_BACKEND_WS_ORIGIN_CHECK")
        .cloned()
        .unwrap_or_else(|| "cors-like".to_string());
    let cloudflared_token = args_map
        .get("cloudflared-token")
        .cloned()
        .or_else(|| merged_env.get("CORN_SYRUP_BACKEND_CLOUDFLARED_TOKEN").cloned());
    let disable_frame_same_origin =
        parse_bool_flag(merged_env.get("CORN_SYRUP_BACKEND_DISABLE_FRAME_SAMEORIGIN"))
            || parse_bool_flag(args_map.get("disable-frame-sameorigin"));
    let test_mode = parse_bool_flag(args_map.get("test"));

    Ok(AppConfig {
        args: args_map,
        hostname,
        port,
        is_ssl,
        ssl_key,
        ssl_cert,
        ssl_key_passphrase,
        local_websocket_url,
        demo_mode,
        data_dir,
        node_env,
        ws_origin_check,
        cloudflared_token,
        disable_frame_same_origin,
        test_mode,
    })
}
