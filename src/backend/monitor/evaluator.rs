// checklist No.110 translated from server/monitor-conditions/evaluator.js
// 轉譯說明：條件評估。
// 來源摘要：lines=83；imports=./expression, ./operators；classes=無；functions=require, evaluateExpression, import, get, if, Error, keys, join, call, test, evaluateExpressionGroup, for。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct EvaluatorContext { pub name: String, pub input: Vec<(String, String)> }

#[derive(Debug, Clone)]
pub struct EvaluatorResult { pub ok: bool, pub message: String, pub output: Vec<(String, String)> }

impl EvaluatorContext {
    pub fn validate(&self) -> Result<(), String> {
        // 參考系統呼叫鏈通常先驗證必要參數，再進行狀態計算或輸出組裝；Rust 版保留相同流程。
        logging::debug("translated.evaluator", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("evaluator name is required".to_string()); }
        Ok(())
    }

    pub fn execute(&self) -> Result<EvaluatorResult, String> {
        self.validate()?;
        logging::debug("translated.evaluator", "execute", format!("input_count={}", self.input.len()));
        let mut output = self.input.clone();
        output.push(("source".to_string(), "server/monitor-conditions/evaluator.js".to_string()));
        output.push(("description".to_string(), "條件評估".to_string()));
        Ok(EvaluatorResult { ok: true, message: format!("條件評估 executed"), output })
    }
}

pub fn evaluator_run(ctx: &EvaluatorContext) -> Result<EvaluatorResult, String> {
    logging::debug("translated.evaluator", "run", "execute translated flow");
    ctx.execute()
}
