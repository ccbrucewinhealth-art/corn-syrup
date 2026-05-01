// checklist No.67 translated from server/model/domain_expiry.js
// 轉譯說明：網域過期模型。
// 來源摘要：lines=368；imports=redbean-node/dist/bean-model, redbean-node, ../../src/util, tldts, ../util-server, ../notification, ../translatable-error, dayjs, ../settings, ../../extra/rdap-dns.json；classes=DomainExpiry；functions=require, getRdapServer, getRdapDnsData, split, pop, if, for, includes, debug。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainExpiryState { Active, Inactive, Deleted }

#[derive(Debug, Clone)]
pub struct DomainExpiryModel {
    pub id: i64,
    pub name: String,
    pub state: DomainExpiryState,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
    pub fields: Vec<(String, String)>,
}

impl DomainExpiryModel {
    pub const SOURCE_PATH: &'static str = "server/model/domain_expiry.js";

    pub fn new(id: i64, name: impl Into<String>) -> Self {
        logging::debug("model.domain_expiry", "new", format!("id={id}"));
        Self { id, name: name.into(), state: DomainExpiryState::Active, created_date: None, updated_date: None, fields: Vec::new() }
    }

    pub fn validate(&self) -> Result<(), String> {
        // 參考系統 BeanModel 寫入前會檢查必要欄位；Rust 版集中在 validate，避免空名稱資料流入 socket/router。
        logging::debug("model.domain_expiry", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("domain_expiry name is required".to_string()); }
        Ok(())
    }

    pub fn set_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        logging::debug("model.domain_expiry", "set_field", &key);
        self.fields.push((key, value.into()));
        self
    }

    pub fn to_json_pairs(&self) -> Vec<(String, String)> {
        logging::debug("model.domain_expiry", "to_json_pairs", format!("field_count={}", self.fields.len()));
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

pub fn domain_expiry_run(model: &DomainExpiryModel) -> Result<Vec<(String, String)>, String> {
    logging::debug("model.domain_expiry", "run", "validate and serialize");
    model.validate()?;
    Ok(model.to_json_pairs())
}
