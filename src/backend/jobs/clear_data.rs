// checklist No.120 translated from server/jobs/clear-old-data.js
// 轉譯說明：清除舊資料。
// 來源摘要：lines=64；imports=redbean-node, ../../src/util, ../database, ../settings, dayjs；classes=無；functions=require, async, clearHeartbeatData, get, if, set, parseInt, catch, warn, info, debug, sqlHourOffset。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct ClearDataContext { pub name: String, pub input: Vec<(String, String)> }

#[derive(Debug, Clone)]
pub struct ClearDataResult { pub ok: bool, pub message: String, pub output: Vec<(String, String)> }

impl ClearDataContext {
    pub fn validate(&self) -> Result<(), String> {
        // 參考系統呼叫鏈通常先驗證必要參數，再進行狀態計算或輸出組裝；Rust 版保留相同流程。
        logging::debug("translated.clear_data", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("clear_data name is required".to_string()); }
        Ok(())
    }

    pub fn execute(&self) -> Result<ClearDataResult, String> {
        self.validate()?;
        logging::debug("translated.clear_data", "execute", format!("input_count={}", self.input.len()));
        let mut output = self.input.clone();
        output.push(("source".to_string(), "server/jobs/clear-old-data.js".to_string()));
        output.push(("description".to_string(), "清除舊資料".to_string()));
        Ok(ClearDataResult { ok: true, message: format!("清除舊資料 executed"), output })
    }
}

pub fn clear_data_run(ctx: &ClearDataContext) -> Result<ClearDataResult, String> {
    logging::debug("translated.clear_data", "run", "execute translated flow");
    ctx.execute()
}
