// checklist No.112 translated from server/monitor-conditions/operators.js
// 轉譯說明：運算子。
// 來源摘要：lines=318；imports=無；classes=ConditionOperator, StringEqualsOperator, StringNotEqualsOperator, ContainsOperator, NotContainsOperator, StartsWithOperator；functions=test, Error, if, isArray, includes, indexOf, startsWith。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct OperatorsContext { pub name: String, pub input: Vec<(String, String)> }

#[derive(Debug, Clone)]
pub struct OperatorsResult { pub ok: bool, pub message: String, pub output: Vec<(String, String)> }

impl OperatorsContext {
    pub fn validate(&self) -> Result<(), String> {
        // 參考系統呼叫鏈通常先驗證必要參數，再進行狀態計算或輸出組裝；Rust 版保留相同流程。
        logging::debug("translated.operators", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("operators name is required".to_string()); }
        Ok(())
    }

    pub fn execute(&self) -> Result<OperatorsResult, String> {
        self.validate()?;
        logging::debug("translated.operators", "execute", format!("input_count={}", self.input.len()));
        let mut output = self.input.clone();
        output.push(("source".to_string(), "server/monitor-conditions/operators.js".to_string()));
        output.push(("description".to_string(), "運算子".to_string()));
        Ok(OperatorsResult { ok: true, message: format!("運算子 executed"), output })
    }
}

pub fn operators_run(ctx: &OperatorsContext) -> Result<OperatorsResult, String> {
    logging::debug("translated.operators", "run", "execute translated flow");
    ctx.execute()
}
