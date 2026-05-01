use std::collections::BTreeMap;
use crate::backend::logging;

#[derive(Debug, Clone, Default)]
pub struct PrometheusLabels(pub BTreeMap<String, String>);

#[derive(Debug, Clone)]
pub struct PrometheusMetric {
    pub name: String,
    pub help: String,
    pub labels: PrometheusLabels,
    pub value: f64,
}

pub fn sanitize_for_prometheus(text: &str) -> String {
    logging::debug("auto.mod", "sanitize_for_prometheus", "enter");
    let filtered: String = text
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    filtered
        .trim_start_matches(|c: char| !c.is_ascii_alphabetic() && c != '_')
        .to_string()
}

pub fn map_tags_to_labels(tags: &[(String, Option<String>)]) -> BTreeMap<String, Vec<String>> {
    logging::debug("auto.mod", "map_tags_to_labels", "enter");
    let mut mapped: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, value) in tags {
        let key = sanitize_for_prometheus(name);
        if key.is_empty() {
            continue;
        }
        let val = sanitize_for_prometheus(value.as_deref().unwrap_or(""));
        if !val.is_empty() {
            mapped.entry(key).or_default().push(val);
        }
    }
    for values in mapped.values_mut() {
        values.sort();
    }
    mapped
}

pub fn monitor_status_metric(labels: PrometheusLabels, status: i32) -> PrometheusMetric {
    logging::debug("auto.mod", "monitor_status_metric", "enter");
    // 對齊 prometheus.js monitor_status Gauge：狀態值 1/0/2/3 直接寫入 metric。
    PrometheusMetric {
        name: "monitor_status".to_string(),
        help: "Monitor Status (1 = UP, 0= DOWN, 2= PENDING, 3= MAINTENANCE)".to_string(),
        labels,
        value: status as f64,
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModContext;

pub fn mod_run(_: &ModContext) -> Result<String, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    Ok("prometheus-registry-ready".to_string())
}
