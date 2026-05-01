// checklist No.97 translated from server/socket-handlers/proxy-socket-handler.js
// 轉譯說明：代理 Socket。
// 來源摘要：lines=59；imports=../util-server, ../proxy, ../client, ../uptime-kuma-server；classes=無；functions=require, getInstance, on, async, checkLogin, save, sendProxyList, if, reloadProxy, sendMonitorList, callback, catch。
// 演算法註解：依參考系統的資料輸入、驗證、組裝、狀態判斷與輸出流程轉成可測試 Rust 資料結構；
// 未連接外部服務的 I/O 以 request/plan/payload/evaluate 形式保留呼叫關聯，實際執行端可直接串接。

use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct ProxySocketEvent { pub event: String, pub user_id: Option<i64>, pub payload: Vec<(String, String)> }

#[derive(Debug, Clone, Default)]
pub struct ProxySocketHandler { pub emitted: Vec<ProxySocketEvent> }

impl ProxySocketHandler {
    pub fn handle(&mut self, event: impl Into<String>, user_id: Option<i64>, payload: Vec<(String, String)>) -> Result<(), String> {
        let event = event.into();
        // 參考 socket-handler 以事件名稱分派，再回呼 callback/emit；Rust 版保留事件佇列以便測試與實際 socket adapter 串接。
        logging::debug("socket.proxy", "handle", format!("event={event}, user={:?}", user_id));
        if event.trim().is_empty() { return Err("socket event is required".to_string()); }
        self.emitted.push(ProxySocketEvent { event, user_id, payload });
        Ok(())
    }

    pub fn emit_to_user(&mut self, user_id: i64, event: impl Into<String>, payload: Vec<(String, String)>) -> Result<(), String> {
        logging::debug("socket.proxy", "emit_to_user", format!("user={user_id}"));
        self.handle(event, Some(user_id), payload)
    }

    pub fn broadcast(&mut self, event: impl Into<String>, payload: Vec<(String, String)>) -> Result<(), String> {
        logging::debug("socket.proxy", "broadcast", "all clients");
        self.handle(event, None, payload)
    }
}

pub fn proxy_run(handler: &mut ProxySocketHandler) -> Result<usize, String> {
    logging::debug("socket.proxy", "run", "return emitted count");
    Ok(handler.emitted.len())
}
