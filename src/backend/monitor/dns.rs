use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorState { Up, Down, Pending }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsRecordType { A, AAAA, CNAME, MX, NS, TXT, CAA, SOA, SRV, PTR, Other(String) }

#[derive(Debug, Clone)]
pub struct DnsRecord { pub record_type: DnsRecordType, pub value: String, pub priority: Option<u16> }

#[derive(Debug, Clone)]
pub struct DnsContext {
    pub source_path: &'static str,
    pub name: String,
    pub target: String,
    pub resolver: Option<String>,
    pub record_type: DnsRecordType,
    pub expected: Option<String>,
    pub observed: Option<String>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct DnsResult { pub state: MonitorState, pub message: String, pub ping_ms: Option<u128>, pub records: Vec<DnsRecord> }

impl Default for DnsContext {
    fn default() -> Self {
        Self { source_path: "server/monitor-types/dns.js", name: "dns".to_string(), target: String::new(), resolver: None, record_type: DnsRecordType::A, expected: None, observed: None, timeout_ms: 30_000 }
    }
}

impl DnsContext {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.dns", "validate", "enter");
        if self.target.trim().is_empty() { return Err(format!("{} target is required", self.name)); }
        if self.timeout_ms == 0 { return Err(format!("{} timeout must be greater than zero", self.name)); }
        Ok(())
    }
}

pub fn format_dns_records(records: &[DnsRecord]) -> String {
    logging::debug("auto.dns", "format_dns_records", "enter");
    if records.is_empty() { return "No records".to_string(); }
    match records[0].record_type {
        DnsRecordType::MX => records.iter().map(|r| format!("Hostname: {} - Priority: {}", r.value, r.priority.unwrap_or_default())).collect::<Vec<_>>().join(" | "),
        DnsRecordType::NS => format!("Servers: {}", records.iter().map(|r| r.value.clone()).collect::<Vec<_>>().join(" | ")),
        DnsRecordType::TXT => format!("Records: {}", records.iter().map(|r| r.value.clone()).collect::<Vec<_>>().join(" | ")),
        DnsRecordType::CAA => format!("Records: {}", records.iter().map(|r| r.value.clone()).filter(|v| !v.is_empty()).collect::<Vec<_>>().join(" | ")),
        _ => records.iter().map(|r| r.value.clone()).collect::<Vec<_>>().join(" | "),
    }
}

pub fn evaluate_dns_records(ctx: &DnsContext, records: Vec<DnsRecord>, elapsed_ms: u128) -> Result<DnsResult, String> {
    logging::debug("auto.dns", "evaluate_dns_records", "enter");
    ctx.validate()?;
    let message = format_dns_records(&records);
    let ok = ctx.expected.as_ref().map(|expected| records.iter().any(|r| r.value.contains(expected))).unwrap_or(!records.is_empty());
    Ok(DnsResult { state: if ok { MonitorState::Up } else { MonitorState::Down }, message: if ok { message } else { format!("DNS expected condition not matched: {message}") }, ping_ms: Some(elapsed_ms), records })
}

pub fn dns_run(ctx: &DnsContext) -> Result<DnsResult, String> {
    logging::debug("auto.dns", "dns_run", "enter");
    ctx.validate()?;
    let observed = ctx.observed.as_deref().unwrap_or(ctx.target.as_str()).to_string();
    let record = DnsRecord { record_type: ctx.record_type.clone(), value: observed, priority: None };
    evaluate_dns_records(ctx, vec![record], 0)
}
