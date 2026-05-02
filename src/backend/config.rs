use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::backend::logging;

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

pub fn load_dotenv_from(code_dir: impl AsRef<Path>) -> Vec<(String, String)> {
    logging::debug("auto.config", "load_dotenv_from", "enter");
    let code_dir = code_dir.as_ref();
    let parent_env = code_dir.parent().map(|p| p.join(".env"));
    let local_env = code_dir.join(".env");
    let env_path = parent_env.filter(|p| p.exists()).unwrap_or(local_env);

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
    let mut merged_env = env_map(&load_dotenv_from("src"));
    merged_env.extend(env_map(env));

    let hostname = args_map
        .get("host")
        .cloned()
        .or_else(|| merged_env.get("UPTIME_KUMA_HOST").cloned())
        .or_else(|| merged_env.get("HOST").cloned());

    let port = args_map
        .get("port")
        .or_else(|| merged_env.get("UPTIME_KUMA_PORT"))
        .or_else(|| merged_env.get("PORT"))
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3001);

    let ssl_key = args_map
        .get("ssl-key")
        .cloned()
        .or_else(|| merged_env.get("UPTIME_KUMA_SSL_KEY").cloned())
        .or_else(|| merged_env.get("SSL_KEY").cloned());
    let ssl_cert = args_map
        .get("ssl-cert")
        .cloned()
        .or_else(|| merged_env.get("UPTIME_KUMA_SSL_CERT").cloned())
        .or_else(|| merged_env.get("SSL_CERT").cloned());
    let ssl_key_passphrase = args_map
        .get("ssl-key-passphrase")
        .cloned()
        .or_else(|| merged_env.get("UPTIME_KUMA_SSL_KEY_PASSPHRASE").cloned())
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
        .get("UPTIME_KUMA_WS_ORIGIN_CHECK")
        .cloned()
        .unwrap_or_else(|| "cors-like".to_string());
    let cloudflared_token = args_map
        .get("cloudflared-token")
        .cloned()
        .or_else(|| merged_env.get("UPTIME_KUMA_CLOUDFLARED_TOKEN").cloned());
    let disable_frame_same_origin =
        parse_bool_flag(merged_env.get("UPTIME_KUMA_DISABLE_FRAME_SAMEORIGIN"))
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
