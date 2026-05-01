use crate::backend::config::AppConfig;
use std::collections::HashMap;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct ServerCoreContext {
    pub source_path: &'static str,
    pub config: AppConfig,
    pub node_env: String,
}

#[derive(Debug, Clone)]
pub struct ServerCore {
    pub server_type: ServerType,
    pub user_agent: String,
    pub index_html: Option<String>,
    pub monitor_type_list: HashMap<String, MonitorTypeDescriptor>,
    pub socket_cors_origin: Option<String>,
    pub axios_timeout_ms: u64,
    pub entry_page: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerType {
    Http,
    Https,
}

#[derive(Debug, Clone)]
pub struct MonitorTypeDescriptor {
    pub id: String,
    pub supports_conditions: bool,
    pub condition_variables: Vec<String>,
}

impl ServerCore {
    pub fn get_user_agent(version: &str) -> String {
        logging::debug("auto.server_core", "get_user_agent", "enter");
        format!("Uptime-Kuma/{version}")
    }

    pub fn default_monitor_types(user_agent: &str) -> HashMap<String, MonitorTypeDescriptor> {
        logging::debug("auto.server_core", "default_monitor_types", "enter");
        let mut map = HashMap::new();
        let items = [
            ("real-browser", true),
            ("tailscale-ping", false),
            ("websocket-upgrade", false),
            ("dns", false),
            ("postgres", false),
            ("mqtt", false),
            ("smtp", false),
            ("group", false),
            ("snmp", false),
            ("grpc-keyword", false),
            ("mongodb", false),
            ("rabbitmq", false),
            ("sip-options", false),
            ("gamedig", false),
            ("port", false),
            ("manual", false),
            ("globalping", false),
            ("redis", false),
            ("system-service", false),
            ("sqlserver", false),
            ("mysql", false),
            ("oracledb", false),
        ];

        // uptime-kuma-server.js 建構子會集中註冊 monitorTypeList；Rust 版以描述物件保留同樣 key，
        // 讓 socket/client 的 sendMonitorTypeList 可依同一份註冊表輸出支援條件與變數資訊。
        for (id, supports_conditions) in items {
            let condition_variables = if id == "globalping" {
                vec![format!("user-agent:{user_agent}")]
            } else {
                Vec::new()
            };
            map.insert(
                id.to_string(),
                MonitorTypeDescriptor {
                    id: id.to_string(),
                    supports_conditions,
                    condition_variables,
                },
            );
        }

        map
    }

    pub fn get_timezone_offset_minutes() -> i32 {
        logging::debug("auto.server_core", "get_timezone_offset_minutes", "enter");
        0
    }

    pub fn get_timezone() -> &'static str {
        logging::debug("auto.server_core", "get_timezone", "enter");
        "UTC"
    }
}

impl ServerCoreContext {
    pub fn new(config: AppConfig) -> Self {
        logging::debug("auto.server_core", "new", "enter");
        let node_env = config.node_env.clone();
        Self {
            source_path: "server/uptime-kuma-server.js",
            config,
            node_env,
        }
    }
}

pub fn server_core_run(ctx: &ServerCoreContext) -> Result<ServerCore, String> {
    logging::debug("auto.server_core", "server_core_run", "enter");
    let version = env!("CARGO_PKG_VERSION");
    let user_agent = ServerCore::get_user_agent(version);
    let server_type = if ctx.config.is_ssl {
        if ctx.config.ssl_key.is_none() || ctx.config.ssl_cert.is_none() {
            return Err("ssl enabled but ssl key/cert missing".to_string());
        }
        ServerType::Https
    } else {
        ServerType::Http
    };

    let index_html = std::fs::read_to_string("./dist/index.html").ok();
    if index_html.is_none() && ctx.node_env != "development" {
        // 參考系統在 production 找不到 dist/index.html 時會 process.exit(1)；
        // Rust 版回傳 Err，讓上層 main_run 可決定是否中止服務或進入安裝檢查流程。
        return Err("cannot find ./dist/index.html outside development mode".to_string());
    }

    let socket_cors_origin = if ctx.node_env == "development" {
        Some("*".to_string())
    } else {
        None
    };

    Ok(ServerCore {
        server_type,
        user_agent: user_agent.clone(),
        index_html,
        monitor_type_list: ServerCore::default_monitor_types(&user_agent),
        socket_cors_origin,
        axios_timeout_ms: 300 * 1000,
        entry_page: "dashboard".to_string(),
    })
}
