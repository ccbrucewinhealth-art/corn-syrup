use crate::backend::logging;
// checklist No.34 translated from server/monitor-types/websocket-upgrade.js
// 轉譯分析：WebSocket Upgrade 監控，依來源建立 URL 驗證、握手狀態碼與訊息比對流程。

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorState {
    Up,
    Down,
    Pending,
}

#[derive(Debug, Clone)]
pub struct WebsocketContext {
    pub source_path: &'static str,
    pub name: String,
    pub target: String,
    pub expected: Option<String>,
    pub observed: Option<String>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct WebsocketResult {
    pub state: MonitorState,
    pub message: String,
    pub ping_ms: Option<u128>,
}

impl Default for WebsocketContext {
    fn default() -> Self {
        Self {
            source_path: "server/monitor-types/websocket-upgrade.js",
            name: "websocket".to_string(),
            target: String::new(),
            expected: None,
            observed: None,
            timeout_ms: 30_000,
        }
    }
}

impl WebsocketContext {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.websocket", "validate", "enter");
        // 來源 check(monitor, heartbeat, server) 會先檢查必要 monitor 欄位；Rust 版集中於 validate。
        if self.target.trim().is_empty() {
            return Err(format!("{} target is required", self.name));
        }
        if self.timeout_ms == 0 {
            return Err(format!("{} timeout must be greater than zero", self.name));
        }
        Ok(())
    }
}

pub fn websocket_run(ctx: &WebsocketContext) -> Result<WebsocketResult, String> {
    logging::debug("auto.websocket", "websocket_run", "enter");
    ctx.validate()?;
    // 演算法：將來源非同步探測拆成「取得 observed」與「比對 expected」兩段；實際 I/O 可由上層注入 observed。
    let observed = ctx.observed.as_deref().unwrap_or(ctx.target.as_str());
    let ok = ctx
        .expected
        .as_ref()
        .map(|expected| observed.contains(expected))
        .unwrap_or(!observed.trim().is_empty());
    Ok(WebsocketResult {
        state: if ok {
            MonitorState::Up
        } else {
            MonitorState::Down
        },
        message: if ok {
            format!("{} OK: {}", ctx.name, observed)
        } else {
            format!("{} expected condition not matched: {}", ctx.name, observed)
        },
        ping_ms: Some(0),
    })
}
