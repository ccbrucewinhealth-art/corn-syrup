use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorState { Up, Down, Pending }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlobalpingSubtype { Ping, Http, Dns }

#[derive(Debug, Clone)]
pub struct GlobalpingContext {
    pub source_path: &'static str,
    pub name: String,
    pub target: String,
    pub location: String,
    pub subtype: GlobalpingSubtype,
    pub protocol: Option<String>,
    pub port: Option<u16>,
    pub expected: Option<String>,
    pub observed: Option<String>,
    pub timeout_ms: u64,
    pub api_token_present: bool,
}

#[derive(Debug, Clone)]
pub struct GlobalpingProbeResult { pub location: String, pub success: bool, pub output: String, pub latency_ms: Option<u128>, pub status_code: Option<u16> }

#[derive(Debug, Clone)]
pub struct GlobalpingResult { pub state: MonitorState, pub message: String, pub ping_ms: Option<u128>, pub probes: Vec<GlobalpingProbeResult> }

impl Default for GlobalpingContext {
    fn default() -> Self {
        Self { source_path: "server/monitor-types/globalping.js", name: "globalping".to_string(), target: String::new(), location: "world".to_string(), subtype: GlobalpingSubtype::Ping, protocol: None, port: None, expected: None, observed: None, timeout_ms: 30_000, api_token_present: false }
    }
}

impl GlobalpingContext {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.globalping", "validate", "enter");
        if self.target.trim().is_empty() { return Err(format!("{} target is required", self.name)); }
        if self.location.trim().is_empty() { return Err("globalping location is required".to_string()); }
        if self.timeout_ms == 0 { return Err(format!("{} timeout must be greater than zero", self.name)); }
        Ok(())
    }

    pub fn measurement_options(&self) -> Vec<(String, String)> {
        logging::debug("auto.globalping", "measurement_options", "enter");
        let mut opts = vec![("target".to_string(), self.target.clone()), ("location".to_string(), self.location.clone()), ("limit".to_string(), "1".to_string())];
        opts.push(("type".to_string(), match self.subtype { GlobalpingSubtype::Ping => "ping", GlobalpingSubtype::Http => "http", GlobalpingSubtype::Dns => "dns" }.to_string()));
        if let Some(protocol) = &self.protocol { opts.push(("protocol".to_string(), protocol.clone())); }
        if let Some(port) = self.port { opts.push(("port".to_string(), port.to_string())); }
        opts
    }
}

pub fn evaluate_globalping(ctx: &GlobalpingContext, probes: Vec<GlobalpingProbeResult>) -> Result<GlobalpingResult, String> {
    logging::debug("auto.globalping", "evaluate_globalping", "enter");
    ctx.validate()?;
    let success = probes.iter().any(|p| p.success);
    let expected_ok = ctx.expected.as_ref().map(|expected| probes.iter().any(|p| p.output.contains(expected))).unwrap_or(true);
    let ok = success && expected_ok;
    let ping_ms = probes.iter().filter_map(|p| p.latency_ms).min();
    let message = if ok { format!("GlobalPing {:?} OK from {}", ctx.subtype, ctx.location) } else { format!("GlobalPing check failed: success={success}, expected_ok={expected_ok}") };
    Ok(GlobalpingResult { state: if ok { MonitorState::Up } else { MonitorState::Down }, message, ping_ms, probes })
}

pub fn globalping_run(ctx: &GlobalpingContext) -> Result<GlobalpingResult, String> {
    logging::debug("auto.globalping", "globalping_run", "enter");
    ctx.validate()?;
    let observed = ctx.observed.as_deref().unwrap_or(ctx.target.as_str()).to_string();
    let probe = GlobalpingProbeResult { location: ctx.location.clone(), success: !observed.trim().is_empty(), output: observed, latency_ms: Some(0), status_code: None };
    evaluate_globalping(ctx, vec![probe])
}
