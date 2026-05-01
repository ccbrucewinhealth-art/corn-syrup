use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorState {
    Up,
    Down,
    Pending,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorTypeName {
    Http,
    Tcp,
    Dns,
    Push,
    Group,
    Globalping,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct MonitorModel {
    pub id: i64,
    pub name: String,
    pub monitor_type: MonitorTypeName,
    pub url: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub active: bool,
    pub upside_down: bool,
    pub interval_seconds: u64,
    pub retry_interval_seconds: u64,
    pub resend_interval: u64,
    pub max_retries: u32,
    pub accepted_statuscodes: Vec<String>,
    pub created_date: Option<String>,
    pub tags: Vec<String>,
    pub notification_ids: Vec<i64>,
    pub extra: Vec<(String, String)>,
}

impl MonitorModel {
    pub const SOURCE_PATH: &'static str = "server/model/monitor.js";

    pub fn new(id: i64, name: impl Into<String>) -> Self {
        logging::debug("auto.monitor", "new", "enter");
        Self {
            id,
            name: name.into(),
            monitor_type: MonitorTypeName::Http,
            url: None,
            hostname: None,
            port: None,
            active: true,
            upside_down: false,
            interval_seconds: 60,
            retry_interval_seconds: 60,
            resend_interval: 0,
            max_retries: 0,
            accepted_statuscodes: vec!["200-299".to_string()],
            created_date: None,
            tags: Vec::new(),
            notification_ids: Vec::new(),
            extra: Vec::new(),
        }
    }

    pub fn set_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        logging::debug("auto.monitor", "set_extra", "enter");
        self.extra.push((key.into(), value.into()));
        self
    }

    pub fn status(&self) -> &'static str {
        logging::debug("auto.monitor", "status", "enter");
        if self.active { "active" } else { "inactive" }
    }

    pub fn display_type(&self) -> String {
        logging::debug("auto.monitor", "display_type", "enter");
        match &self.monitor_type {
            MonitorTypeName::Http => "http".to_string(),
            MonitorTypeName::Tcp => "tcp".to_string(),
            MonitorTypeName::Dns => "dns".to_string(),
            MonitorTypeName::Push => "push".to_string(),
            MonitorTypeName::Group => "group".to_string(),
            MonitorTypeName::Globalping => "globalping".to_string(),
            MonitorTypeName::Custom(v) => v.clone(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.monitor", "validate", "enter");
        if self.name.trim().is_empty() { return Err("monitor name is required".to_string()); }
        if self.interval_seconds == 0 { return Err("monitor interval must be greater than zero".to_string()); }
        match self.monitor_type {
            MonitorTypeName::Http => {
                let url = self.url.as_deref().unwrap_or_default();
                if !(url.starts_with("http://") || url.starts_with("https://")) {
                    return Err("http monitor url must start with http:// or https://".to_string());
                }
            }
            MonitorTypeName::Tcp | MonitorTypeName::Dns | MonitorTypeName::Globalping => {
                if self.hostname.as_deref().unwrap_or_default().trim().is_empty() {
                    return Err("monitor hostname is required".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn heartbeat_state(&self, raw_up: bool) -> MonitorState {
        logging::debug("auto.monitor", "heartbeat_state", "enter");
        let up = if self.upside_down { !raw_up } else { raw_up };
        if up { MonitorState::Up } else { MonitorState::Down }
    }

    pub fn to_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("auto.monitor", "to_json_pairs", "enter");
        let mut out = vec![
            ("id".to_string(), self.id.to_string()),
            ("name".to_string(), self.name.clone()),
            ("type".to_string(), self.display_type()),
            ("active".to_string(), self.active.to_string()),
            ("status".to_string(), self.status().to_string()),
            ("interval".to_string(), self.interval_seconds.to_string()),
            ("retryInterval".to_string(), self.retry_interval_seconds.to_string()),
            ("resendInterval".to_string(), self.resend_interval.to_string()),
            ("maxretries".to_string(), self.max_retries.to_string()),
            ("accepted_statuscodes".to_string(), self.accepted_statuscodes.join(",")),
        ];
        if let Some(url) = &self.url { out.push(("url".to_string(), url.clone())); }
        if let Some(hostname) = &self.hostname { out.push(("hostname".to_string(), hostname.clone())); }
        if let Some(port) = self.port { out.push(("port".to_string(), port.to_string())); }
        if let Some(created) = &self.created_date { out.push(("createdDate".to_string(), created.clone())); }
        out.push(("tags".to_string(), self.tags.join(",")));
        out.extend(self.extra.clone());
        out
    }

    pub fn to_public_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("auto.monitor", "to_public_json_pairs", "enter");
        self.to_json_pairs()
            .into_iter()
            .filter(|(k, _)| !matches!(k.as_str(), "notificationIDList" | "databaseConnectionString" | "mqttPassword"))
            .collect()
    }
}
