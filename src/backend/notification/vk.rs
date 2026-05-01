// checklist No.217 translated from server/notification-providers/vk.js
// 補強說明：依參考系統 notification provider 的 send() 流程補足 validate、payload 組裝、HTTP request plan 與 debug log。
// 演算法：先驗證通知設定與訊息，再將 monitor/heartbeat/message 組成 provider payload，最後回傳可由 HTTP client 執行的 request plan。

use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationMessage {
    pub title: String,
    pub body: String,
    pub url: Option<String>,
    pub status: Option<String>,
    pub monitor_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationPayload {
    pub provider: &'static str,
    pub endpoint: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkNotification {
    pub endpoint: Option<String>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub recipient: Option<String>,
    pub extra_fields: Vec<(String, String)>,
}

impl VkNotification {
    pub const SOURCE_PATH: &'static str = "server/notification-providers/vk.js";
    pub const PROVIDER: &'static str = "vk";

    pub fn new(endpoint: Option<String>, token: Option<String>) -> Self {
        logging::debug("notification.vk", "new", "create provider configuration");
        Self { endpoint, token, username: None, password: None, recipient: None, extra_fields: Vec::new() }
    }

    pub fn validate(&self, message: &NotificationMessage) -> Result<(), String> {
        logging::debug("notification.vk", "validate", format!("title_len={}, body_len={}", message.title.len(), message.body.len()));
        if message.title.trim().is_empty() && message.body.trim().is_empty() { return Err("notification message is empty".to_string()); }
        if self.endpoint.as_deref().unwrap_or_default().trim().is_empty() { return Err(format!("{} endpoint is required", Self::PROVIDER)); }
        Ok(())
    }

    pub fn normalized_endpoint(&self) -> String {
        logging::debug("notification.vk", "normalized_endpoint", "resolve endpoint fallback");
        self.endpoint.clone().unwrap_or_else(|| format!("provider://{}", Self::PROVIDER))
    }

    pub fn build_fields(&self, message: &NotificationMessage) -> Vec<(String, String)> {
        // 參考系統各 provider 會把 msg、monitor、heartbeat 狀態攤平成 POST body；這裡保留相同資料關聯。
        logging::debug("notification.vk", "build_fields", "compose title/body/status/monitor fields");
        let mut fields = vec![("title".to_string(), message.title.clone()), ("message".to_string(), message.body.clone())];
        if let Some(url)=&message.url { fields.push(("url".to_string(), url.clone())); }
        if let Some(status)=&message.status { fields.push(("status".to_string(), status.clone())); }
        if let Some(monitor)=&message.monitor_name { fields.push(("monitor".to_string(), monitor.clone())); }
        if let Some(recipient)=&self.recipient { fields.push(("recipient".to_string(), recipient.clone())); }
        if let Some(username)=&self.username { fields.push(("username".to_string(), username.clone())); }
        fields.extend(self.extra_fields.clone());
        fields
    }

    pub fn build_payload(&self, message: &NotificationMessage) -> Result<NotificationPayload, String> {
        self.validate(message)?;
        logging::debug("notification.vk", "build_payload", "build HTTP request payload");
        let mut headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        if let Some(token)=&self.token { headers.push(("Authorization".to_string(), format!("Bearer {token}"))); }
        if let Some(password)=&self.password { headers.push(("X-Provider-Password".to_string(), password.clone())); }
        Ok(NotificationPayload { provider: Self::PROVIDER, endpoint: self.normalized_endpoint(), method: "POST".to_string(), headers, fields: self.build_fields(message) })
    }

    pub fn send_plan(&self, message: &NotificationMessage) -> Result<Vec<(String, String)>, String> {
        logging::debug("notification.vk", "send_plan", "convert payload to adapter-neutral plan");
        let payload = self.build_payload(message)?;
        let mut plan = vec![("provider".to_string(), payload.provider.to_string()), ("method".to_string(), payload.method), ("endpoint".to_string(), payload.endpoint)];
        plan.extend(payload.headers.into_iter().map(|(k,v)| (format!("header:{k}"), v)));
        plan.extend(payload.fields.into_iter().map(|(k,v)| (format!("field:{k}"), v)));
        Ok(plan)
    }
}

pub fn vk_run(provider: &VkNotification, message: &NotificationMessage) -> Result<NotificationPayload, String> {
    logging::debug("notification.vk", "run", "validate and build provider payload");
    provider.build_payload(message)
}
