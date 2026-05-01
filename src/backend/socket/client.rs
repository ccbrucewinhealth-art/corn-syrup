use std::collections::BTreeMap;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct ClientSocket {
    pub user_id: u64,
    pub emitted: Vec<ClientEvent>,
}

#[derive(Debug, Clone)]
pub struct ClientEvent {
    pub target_user_id: Option<u64>,
    pub event: String,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub platform: String,
    pub arch: String,
}

#[derive(Debug, Clone)]
pub struct ServerInfoPayload {
    pub primary_base_url: Option<String>,
    pub server_timezone: String,
    pub server_timezone_offset: i32,
    pub version: Option<String>,
    pub latest_version: Option<String>,
    pub is_container: bool,
    pub db_type: Option<String>,
    pub runtime: Option<RuntimeInfo>,
}

impl ClientSocket {
    pub fn new(user_id: u64) -> Self {
        logging::debug("auto.client", "new", "enter");
        Self {
            user_id,
            emitted: Vec::new(),
        }
    }

    pub fn emit(&mut self, event: &str, payload: impl Into<String>) {
        logging::debug("auto.client", "emit", "enter");
        self.emitted.push(ClientEvent {
            target_user_id: None,
            event: event.to_string(),
            payload: payload.into(),
        });
    }

    pub fn emit_to_user(&mut self, event: &str, payload: impl Into<String>) {
        logging::debug("auto.client", "emit_to_user", "enter");
        self.emitted.push(ClientEvent {
            target_user_id: Some(self.user_id),
            event: event.to_string(),
            payload: payload.into(),
        });
    }
}

fn serialize_list(list: &[String]) -> String {
    format!(
        "[{}]",
        list.iter()
            .map(|v| format!("\"{v}\""))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub fn send_notification_list(socket: &mut ClientSocket, notifications: &[String]) -> Vec<String> {
    logging::debug("auto.client", "send_notification_list", "enter");
    // client.js 會查詢目前 user_id 的 notification 並透過 io.to(userID) 廣播；
    // Rust 版由呼叫端注入已過濾資料，保留「回傳 list + emit notificationList」呼叫關聯。
    socket.emit_to_user("notificationList", serialize_list(notifications));
    notifications.to_vec()
}

pub fn send_heartbeat_list(
    socket: &mut ClientSocket,
    monitor_id: u64,
    heartbeats_desc: &[String],
    to_user: bool,
    overwrite: bool,
) {
    logging::debug("auto.client", "send_heartbeat_list", "enter");
    let mut result = heartbeats_desc.to_vec();
    result.reverse();
    let payload = format!(
        "{{\"monitorID\":{monitor_id},\"list\":{},\"overwrite\":{overwrite}}}",
        serialize_list(&result)
    );
    if to_user {
        socket.emit_to_user("heartbeatList", payload);
    } else {
        socket.emit("heartbeatList", payload);
    }
}

pub fn send_info(socket: &mut ClientSocket, info: &ServerInfoPayload, hide_version: bool) {
    logging::debug("auto.client", "send_info", "enter");
    let mut fields = BTreeMap::new();
    fields.insert("serverTimezone", format!("\"{}\"", info.server_timezone));
    fields.insert(
        "serverTimezoneOffset",
        info.server_timezone_offset.to_string(),
    );
    if let Some(url) = &info.primary_base_url {
        fields.insert("primaryBaseURL", format!("\"{url}\""));
    }
    if !hide_version {
        if let Some(version) = &info.version {
            fields.insert("version", format!("\"{version}\""));
        }
        if let Some(latest) = &info.latest_version {
            fields.insert("latestVersion", format!("\"{latest}\""));
        }
        fields.insert("isContainer", info.is_container.to_string());
        if let Some(db_type) = &info.db_type {
            fields.insert("dbType", format!("\"{db_type}\""));
        }
    }
    let payload = format!(
        "{{{}}}",
        fields
            .into_iter()
            .map(|(k, v)| format!("\"{k}\":{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    socket.emit("info", payload);
}

pub fn send_monitor_type_list(socket: &mut ClientSocket, monitor_types_json: &str) {
    logging::debug("auto.client", "send_monitor_type_list", "enter");
    socket.emit_to_user("monitorTypeList", monitor_types_json.to_string());
}

#[derive(Debug, Clone)]
pub struct ClientContext {
    pub user_id: u64,
}

pub fn client_run(ctx: &ClientContext) -> Result<ClientSocket, String> {
    logging::debug("auto.client", "client_run", "enter");
    if ctx.user_id == 0 {
        return Err("user id is required".to_string());
    }
    Ok(ClientSocket::new(ctx.user_id))
}
