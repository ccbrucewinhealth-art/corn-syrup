// checklist No.153 translated from server/notification-providers/bark.js
// 轉譯說明：Bark。
// 來源摘要：lines=128；imports=./notification-provider, ../../src/util, axios；classes=Bark；functions=require, send, if, endsWith, substring, postNotification, styles, additionalParameters, checkResult。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct BarkNotification { pub title: String, pub message: String, pub endpoint: Option<String>, pub token: Option<String>, pub fields: Vec<(String, String)> }

#[derive(Debug, Clone)]
pub struct BarkPayload { pub url: String, pub headers: Vec<(String, String)>, pub body: String }

impl BarkNotification {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("notification.bark", "validate", &self.title);
        if self.title.trim().is_empty() { return Err("notification title is required".to_string()); }
        if self.message.trim().is_empty() { return Err("notification message is required".to_string()); }
        Ok(())
    }

    pub fn render_body(&self) -> String {
        // 參考通知 provider 會依服務格式組 JSON/form body；此處以穩定 key=value body 保留 title/message/欄位組裝流程。
        logging::debug("notification.bark", "render_body", format!("fields={}", self.fields.len()));
        let mut parts = vec![format!("title={}", self.title), format!("message={}", self.message)];
        parts.extend(self.fields.iter().map(|(k,v)| format!("{k}={v}")));
        parts.join("&")
    }

    pub fn build_payload(&self) -> Result<BarkPayload, String> {
        self.validate()?;
        logging::debug("notification.bark", "build_payload", "compose request payload");
        let mut headers = vec![("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string())];
        if let Some(token)=&self.token { headers.push(("Authorization".to_string(), format!("Bearer {token}"))); }
        Ok(BarkPayload { url: self.endpoint.clone().unwrap_or_else(|| "provider://bark".to_string()), headers, body: self.render_body() })
    }
}

pub fn bark_run(notification: &BarkNotification) -> Result<BarkPayload, String> {
    logging::debug("notification.bark", "run", "validate and build payload");
    notification.build_payload()
}
