use crate::backend::util::{accepted_status_ranges, check_status_code, evaluate_json_query};
use std::collections::BTreeMap;
use std::time::Duration;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct HttpMonitorRequest {
    pub url: String,
    pub method: String,
    pub headers: BTreeMap<String, String>,
    pub body: Option<String>,
    pub timeout: Duration,
    pub accepted_statuscodes: Vec<String>,
    pub keyword: Option<String>,
    pub invert_keyword: bool,
    pub json_path: Option<String>,
    pub auth_method: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HttpMonitorResult {
    pub ok: bool,
    pub status: u16,
    pub message: String,
    pub ping_ms: Option<u128>,
    pub cert_days_remaining: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct HttpResponseSnapshot {
    pub status: u16,
    pub body: String,
    pub elapsed_ms: u128,
    pub cert_expiry_epoch_secs: Option<i64>,
    pub now_epoch_secs: Option<i64>,
}

impl HttpMonitorRequest {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.http", "validate", "enter");
        if !(self.url.starts_with("http://") || self.url.starts_with("https://")) {
            return Err("http monitor url must start with http:// or https://".to_string());
        }
        if self.method.trim().is_empty() {
            return Err("http monitor method is required".to_string());
        }
        if self.timeout.is_zero() {
            return Err("http monitor timeout must be greater than zero".to_string());
        }
        Ok(())
    }

    pub fn accepted_status_text(&self) -> String {
        logging::debug("auto.http", "accepted_status_text", "enter");
        if self.accepted_statuscodes.is_empty() {
            "200-299".to_string()
        } else {
            self.accepted_statuscodes.join("-")
        }
    }

    pub fn build_request_plan(&self) -> Result<Vec<(String, String)>, String> {
        logging::debug("auto.http", "build_request_plan", "enter");
        self.validate()?;
        let mut plan = vec![
            ("url".to_string(), self.url.clone()),
            ("method".to_string(), self.method.to_uppercase()),
            ("timeoutMs".to_string(), self.timeout.as_millis().to_string()),
            ("acceptedStatus".to_string(), self.accepted_status_text()),
        ];
        if let Some(body) = &self.body { plan.push(("body".to_string(), body.clone())); }
        if let Some(auth) = &self.auth_method { plan.push(("authMethod".to_string(), auth.clone())); }
        for (k, v) in &self.headers { plan.push((format!("header:{k}"), v.clone())); }
        Ok(plan)
    }
}

pub fn evaluate_status(status: u16, accepted: &[u16]) -> bool {
    logging::debug("auto.http", "evaluate_status", "enter");
    accepted.contains(&status)
}

pub fn http_run(req: &HttpMonitorRequest, simulated_status: u16) -> Result<HttpMonitorResult, String> {
    logging::debug("auto.http", "http_run", "enter");
    let snapshot = HttpResponseSnapshot {
        status: simulated_status,
        body: String::new(),
        elapsed_ms: 0,
        cert_expiry_epoch_secs: None,
        now_epoch_secs: None,
    };
    evaluate_http_response(req, &snapshot)
}

pub fn evaluate_http_response(req: &HttpMonitorRequest, response: &HttpResponseSnapshot) -> Result<HttpMonitorResult, String> {
    logging::debug("auto.http", "evaluate_http_response", "enter");
    req.validate()?;
    let accepted = if req.accepted_statuscodes.is_empty() {
        accepted_status_ranges(&["200-399"])
    } else {
        accepted_status_ranges(&req.accepted_statuscodes.iter().map(String::as_str).collect::<Vec<_>>())
    };
    let status_ok = if accepted.is_empty() { check_status_code(response.status, "200-399") } else { evaluate_status(response.status, &accepted) };
    let keyword_ok = match &req.keyword {
        Some(keyword) if req.invert_keyword => !response.body.contains(keyword),
        Some(keyword) => response.body.contains(keyword),
        None => true,
    };
    let json_ok = req.json_path.as_ref().map(|q| evaluate_json_query(&response.body, q)).unwrap_or(true);
    let ok = status_ok && keyword_ok && json_ok;
    let cert_days_remaining = match (response.now_epoch_secs, response.cert_expiry_epoch_secs) {
        (Some(now), Some(expiry)) => Some(crate::backend::util::get_days_remaining(now, expiry)),
        _ => None,
    };
    Ok(HttpMonitorResult {
        ok,
        status: response.status,
        message: if ok { format!("HTTP OK ({})", response.status) } else { format!("HTTP check failed: status_ok={status_ok}, keyword_ok={keyword_ok}, json_ok={json_ok}") },
        ping_ms: Some(response.elapsed_ms),
        cert_days_remaining,
    })
}
