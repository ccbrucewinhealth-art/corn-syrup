use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct TcpContext {
    pub hostname: String,
    pub port: u16,
    pub timeout: Duration,
    pub dns_resolve_server: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TcpMonitorResult {
    pub ok: bool,
    pub latency_ms: Option<u128>,
    pub message: String,
    pub resolved_address: Option<String>,
}

impl TcpContext {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.tcp", "validate", "enter");
        if self.hostname.trim().is_empty() { return Err("tcp hostname is required".to_string()); }
        if self.port == 0 { return Err("tcp port is required".to_string()); }
        if self.timeout.is_zero() { return Err("tcp timeout must be greater than zero".to_string()); }
        Ok(())
    }

    pub fn socket_target(&self) -> String {
        logging::debug("auto.tcp", "socket_target", "enter");
        format!("{}:{}", self.hostname, self.port)
    }
}

pub fn tcp_run(ctx: &TcpContext) -> Result<TcpMonitorResult, String> {
    logging::debug("auto.tcp", "tcp_run", "enter");
    ctx.validate()?;
    let addr = (ctx.hostname.as_str(), ctx.port)
        .to_socket_addrs()
        .map_err(|e| format!("resolve tcp target failed: {e}"))?
        .next()
        .ok_or_else(|| "no tcp address resolved".to_string())?;
    let start = Instant::now();
    match TcpStream::connect_timeout(&addr, ctx.timeout) {
        Ok(_) => Ok(TcpMonitorResult { ok: true, latency_ms: Some(start.elapsed().as_millis()), message: "TCP connected".to_string(), resolved_address: Some(addr.to_string()) }),
        Err(e) => Ok(TcpMonitorResult { ok: false, latency_ms: None, message: format!("TCP connect failed: {e}"), resolved_address: Some(addr.to_string()) }),
    }
}

pub fn tcp_evaluate_simulated(ctx: &TcpContext, connected: bool, latency_ms: u128) -> Result<TcpMonitorResult, String> {
    logging::debug("auto.tcp", "tcp_evaluate_simulated", "enter");
    ctx.validate()?;
    Ok(TcpMonitorResult {
        ok: connected,
        latency_ms: connected.then_some(latency_ms),
        message: if connected { "TCP connected" } else { "TCP connect failed" }.to_string(),
        resolved_address: Some(ctx.socket_target()),
    })
}
