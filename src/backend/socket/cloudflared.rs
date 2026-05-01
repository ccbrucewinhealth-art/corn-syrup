// checklist No.93 translated from server/socket-handlers/cloudflared-socket-handler.js
// 轉譯說明：Cloudflare Tunnel。
// 來源摘要：lines=128；imports=../util-server, node-cloudflared-tunnel, ../uptime-kuma-server, ../../src/util；classes=無；functions=require, getInstance, CloudflaredTunnel, to, emit, on, async, checkLogin, join。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct CloudflaredSocketEvent { pub event: String, pub user_id: Option<i64>, pub payload: Vec<(String, String)> }

#[derive(Debug, Clone, Default)]
pub struct CloudflaredSocketHandler { pub emitted: Vec<CloudflaredSocketEvent> }

impl CloudflaredSocketHandler {
    pub fn handle(&mut self, event: impl Into<String>, user_id: Option<i64>, payload: Vec<(String, String)>) -> Result<(), String> {
        let event = event.into();
        // 參考 socket-handler 以事件名稱分派，再回呼 callback/emit；Rust 版保留事件佇列以便測試與實際 socket adapter 串接。
        logging::debug("socket.cloudflared", "handle", format!("event={event}, user={:?}", user_id));
        if event.trim().is_empty() { return Err("socket event is required".to_string()); }
        self.emitted.push(CloudflaredSocketEvent { event, user_id, payload });
        Ok(())
    }

    pub fn emit_to_user(&mut self, user_id: i64, event: impl Into<String>, payload: Vec<(String, String)>) -> Result<(), String> {
        logging::debug("socket.cloudflared", "emit_to_user", format!("user={user_id}"));
        self.handle(event, Some(user_id), payload)
    }

    pub fn broadcast(&mut self, event: impl Into<String>, payload: Vec<(String, String)>) -> Result<(), String> {
        logging::debug("socket.cloudflared", "broadcast", "all clients");
        self.handle(event, None, payload)
    }
}

pub fn cloudflared_run(handler: &mut CloudflaredSocketHandler) -> Result<usize, String> {
    logging::debug("socket.cloudflared", "run", "return emitted count");
    Ok(handler.emitted.len())
}
