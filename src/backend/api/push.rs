use crate::backend::logging;
// checklist No.33 translated from server/monitor-types/manual.js
// 轉譯分析：manual.js 允許使用者手動指定 UP/DOWN/PENDING；push API 需接收外部狀態並轉成 heartbeat。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushStatus {
    Up,
    Down,
    Pending,
}

#[derive(Debug, Clone)]
pub struct PushRequest {
    pub monitor_id: i64,
    pub status: Option<PushStatus>,
    pub message: Option<String>,
    pub ping_ms: Option<u128>,
}

#[derive(Debug, Clone)]
pub struct PushHeartbeat {
    pub monitor_id: i64,
    pub status: PushStatus,
    pub message: String,
    pub ping_ms: Option<u128>,
}

pub fn handle_push(req: PushRequest) -> Result<PushHeartbeat, String> {
    logging::debug("auto.push", "handle_push", "enter");
    if req.monitor_id <= 0 {
        return Err("monitor_id is required".to_string());
    }
    // 來源 manual.js：manual_status 為 null 時為 PENDING；有值時依 UP/DOWN/其他值轉訊息。
    let status = req.status.unwrap_or(PushStatus::Pending);
    let default_message = match status {
        PushStatus::Up => "Up",
        PushStatus::Down => "Down",
        PushStatus::Pending => "Manual monitoring - No status set",
    };
    Ok(PushHeartbeat {
        monitor_id: req.monitor_id,
        status,
        message: req.message.unwrap_or_else(|| default_message.to_string()),
        ping_ms: req.ping_ms,
    })
}
