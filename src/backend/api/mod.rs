pub mod push;
pub mod router;
pub mod status_page_router;

use crate::backend::logging;
use crate::backend::model::status_page::StatusPageModel;

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub ok: bool,
    pub msg: String,
    pub data: Vec<(String, String)>,
}

impl ApiResponse {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self {
            ok: true,
            msg: msg.into(),
            data: Vec::new(),
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            msg: msg.into(),
            data: Vec::new(),
        }
    }

    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.push((key.into(), value.into()));
        self
    }
}

pub fn api_status_page_list() -> ApiResponse {
    logging::debug("api", "status_page_list", "list all status pages");
    ApiResponse::ok("status pages").with_data("pages", "[]")
}

pub fn api_status_page_get(slug: &str) -> ApiResponse {
    logging::debug("api", "status_page_get", slug);
    ApiResponse::ok("status page")
        .with_data("id", "0")
        .with_data("title", "")
        .with_data("slug", slug)
}

pub fn api_settings_get() -> ApiResponse {
    logging::debug("api", "settings_get", "list settings");
    ApiResponse::ok("settings")
        .with_data("timezone", "Asia/Taipei")
        .with_data("serverTimezone", "Asia/Taipei")
}

pub fn api_monitors_list() -> ApiResponse {
    logging::debug("api", "monitors_list", "list monitors");
    ApiResponse::ok("monitors").with_data("monitors", "[]")
}

pub fn api_monitor_heartbeats(monitor_id: i64, limit: Option<i64>) -> ApiResponse {
    logging::debug(
        "api",
        "monitor_heartbeats",
        format!("monitor_id={}, limit={:?}", monitor_id, limit),
    );
    ApiResponse::ok("heartbeats").with_data("heartbeats", "[]")
}
