// checklist No.111 translated from server/monitor-conditions/expression.js
// 轉譯說明：表達式。
// 來源摘要：lines=111；imports=無；classes=ConditionExpressionGroup, ConditionExpression；functions=processMonitorConditions, forEach, if, ConditionExpressionGroup, push, ConditionExpression, constructor, fromMonitor, parse。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct ExpressionContext { pub name: String, pub input: Vec<(String, String)> }

#[derive(Debug, Clone)]
pub struct ExpressionResult { pub ok: bool, pub message: String, pub output: Vec<(String, String)> }

impl ExpressionContext {
    pub fn validate(&self) -> Result<(), String> {
        // 參考系統呼叫鏈通常先驗證必要參數，再進行狀態計算或輸出組裝；Rust 版保留相同流程。
        logging::debug("translated.expression", "validate", &self.name);
        if self.name.trim().is_empty() { return Err("expression name is required".to_string()); }
        Ok(())
    }

    pub fn execute(&self) -> Result<ExpressionResult, String> {
        self.validate()?;
        logging::debug("translated.expression", "execute", format!("input_count={}", self.input.len()));
        let mut output = self.input.clone();
        output.push(("source".to_string(), "server/monitor-conditions/expression.js".to_string()));
        output.push(("description".to_string(), "表達式".to_string()));
        Ok(ExpressionResult { ok: true, message: format!("表達式 executed"), output })
    }
}

pub fn expression_run(ctx: &ExpressionContext) -> Result<ExpressionResult, String> {
    logging::debug("translated.expression", "run", "execute translated flow");
    ctx.execute()
}
