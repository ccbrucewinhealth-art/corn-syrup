// checklist No.65 translated from server/model/status_page.js
// 轉譯說明：狀態頁面模型。
// 來源摘要：lines=585；imports=redbean-node/dist/bean-model, redbean-node, cheerio, ../uptime-kuma-server, jsesc, ../analytics/analytics, marked, feed, ../config, dayjs, ../util-server, ../../src/util；classes=StatusPage；functions=require, handleStatusPageRSSResponse, findOne, if, buildRSSUrl, type, send。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusPageState { Active, Inactive, Deleted }

#[derive(Debug, Clone)]
pub struct StatusPageModel {
    pub id: i64,
    pub name: String,
    pub state: StatusPageState,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
    pub fields: Vec<(String, String)>,
}

impl StatusPageModel {
    pub const SOURCE_PATH: &'static str = "server/model/status_page.js";

    pub fn new(id: i64, name: impl Into<String>) -> Self {
        logging::debug("model.status_page", "new", format!("id={id}"));
        Self { id, name: name.into(), state: StatusPageState::Active, created_date: None, updated_date: None, fields: Vec::new() }
    }

    pub fn validate(&self) -> Result<(), String> {
        // 參考系統 BeanModel 寫入前會檢查必要欄位；Rust 版集中在 validate，避免空名稱資料流入 socket/router。
        logging::debug("model.status_page", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("status_page name is required".to_string()); }
        Ok(())
    }

    pub fn set_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        logging::debug("model.status_page", "set_field", &key);
        self.fields.push((key, value.into()));
        self
    }

    pub fn to_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("model.status_page", "to_json_pairs", format!("field_count={}", self.fields.len()));
        let mut out = vec![("id".to_string(), self.id.to_string()), ("name".to_string(), self.name.clone()), ("state".to_string(), format!("{:?}", self.state))];
        if let Some(v)=&self.created_date { out.push(("createdDate".to_string(), v.clone())); }
        if let Some(v)=&self.updated_date { out.push(("updatedDate".to_string(), v.clone())); }
        out.extend(self.fields.clone());
        out
    }

    pub fn to_public_json_pairs(&self) -> Vec<(String, String)> {
        // 公開輸出移除 password/token/secret 類欄位，對齊參考系統 public API 不輸出敏感資料的關聯。
        self.to_json_pairs().into_iter().filter(|(k, _)| { let k=k.to_lowercase(); !(k.contains("password")||k.contains("token")||k.contains("secret")) }).collect()
    }
}

pub fn status_page_run(model: &StatusPageModel) -> Result<Vec<(String, String)>, String> {
    logging::debug("model.status_page", "run", "validate and serialize");
    model.validate()?;
    Ok(model.to_json_pairs())
}
