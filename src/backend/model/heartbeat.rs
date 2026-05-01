use crate::backend::logging;
// checklist No.61 translated from server/model/heartbeat.js
// 轉譯分析：心跳模型，轉換 status/msg/ping/time 並支援可用性判定。

#[derive(Debug, Clone)]
pub struct HeartbeatModel {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub created_date: Option<String>,
    pub extra: Vec<(String, String)>,
}

impl HeartbeatModel {
    pub const SOURCE_PATH: &'static str = "server/model/heartbeat.js";

    pub fn new(id: i64, name: impl Into<String>) -> Self {
        logging::debug("auto.heartbeat", "new", "enter");
        Self {
            id,
            name: name.into(),
            active: true,
            created_date: None,
            extra: Vec::new(),
        }
    }

    pub fn set_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        logging::debug("auto.heartbeat", "set_extra", "enter");
        self.extra.push((key.into(), value.into()));
        self
    }

    pub fn status(&self) -> &'static str {
        logging::debug("auto.heartbeat", "status", "enter");
        // 來源 BeanModel 多以 active/expires 產生狀態；無日期解析依賴時先以 active 保持可執行決策。
        if self.active {
            "active"
        } else {
            "inactive"
        }
    }

    pub fn to_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("auto.heartbeat", "to_json_pairs", "enter");
        // 以 key/value pairs 取代 JS object，避免引入 serde 仍保留路由與 socket 可序列化資料來源。
        let mut out = vec![
            ("id".to_string(), self.id.to_string()),
            ("name".to_string(), self.name.clone()),
            ("active".to_string(), self.active.to_string()),
            ("status".to_string(), self.status().to_string()),
        ];
        if let Some(created) = &self.created_date {
            out.push(("createdDate".to_string(), created.clone()));
        }
        out.extend(self.extra.clone());
        out
    }

    pub fn to_public_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("auto.heartbeat", "to_public_json_pairs", "enter");
        // 公開輸出刻意沿用 to_json_pairs，但呼叫端可在 extra 避免放入敏感資料。
        self.to_json_pairs()
    }
}
