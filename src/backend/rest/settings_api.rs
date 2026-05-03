use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, QuerySelect, Statement};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::backend::model::setting;

#[derive(Debug, Deserialize)]
pub struct SettingsUpdateRequest {
    #[serde(default)]
    section: Option<String>,
    #[serde(default)]
    settings: Option<Map<String, Value>>,
    #[serde(flatten)]
    direct: Map<String, Value>,
}

impl SettingsUpdateRequest {
    fn to_masked_log_json(&self) -> String {
        let mut root = Map::new();
        if let Some(section) = &self.section {
            root.insert("section".to_string(), Value::String(section.clone()));
        }
        if let Some(settings) = &self.settings {
            root.insert("settings".to_string(), Value::Object(settings.clone()));
        }
        for (key, value) in &self.direct {
            root.insert(key.clone(), value.clone());
        }
        mask_json_for_log(&Value::Object(root).to_string())
    }
}

#[derive(Debug, Clone, Copy)]
enum SettingValueKind {
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone, Copy)]
struct SettingSchema {
    key: &'static str,
    group: &'static str,
    kind: SettingValueKind,
    default_value: &'static str,
    is_secret: bool,
    allowed_values: &'static [&'static str],
}

const SETTINGS_SCHEMA: &[SettingSchema] = &[
    SettingSchema { key: "timezone", group: "general", kind: SettingValueKind::String, default_value: "Auto: Asia/Taipei", is_secret: false, allowed_values: &["Auto: Asia/Taipei", "UTC", "Asia/Taipei"] },
    SettingSchema { key: "serverTimezone", group: "general", kind: SettingValueKind::String, default_value: "(UTC+08:00) Asia/Taipei", is_secret: false, allowed_values: &["(UTC+08:00) Asia/Taipei"] },
    SettingSchema { key: "searchEngineVisibility", group: "general", kind: SettingValueKind::String, default_value: "Allow indexing", is_secret: false, allowed_values: &["Allow indexing", "Discourage search engines from indexing site"] },
    SettingSchema { key: "entryPage", group: "general", kind: SettingValueKind::String, default_value: "Dashboard", is_secret: false, allowed_values: &["Dashboard", "Status Page - aa", "Status Page - ff"] },
    SettingSchema { key: "primaryBaseUrl", group: "general", kind: SettingValueKind::String, default_value: "", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "steamApiKey", group: "general", kind: SettingValueKind::String, default_value: "", is_secret: true, allowed_values: &[] },
    SettingSchema { key: "globalpingApiToken", group: "general", kind: SettingValueKind::String, default_value: "", is_secret: true, allowed_values: &[] },
    SettingSchema { key: "nscdEnabled", group: "general", kind: SettingValueKind::String, default_value: "Disable", is_secret: false, allowed_values: &["Enable", "Disable"] },
    SettingSchema { key: "chromeExecutable", group: "general", kind: SettingValueKind::String, default_value: "", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "language", group: "appearance", kind: SettingValueKind::String, default_value: "繁體中文", is_secret: false, allowed_values: &["繁體中文", "English", "日本語"] },
    SettingSchema { key: "theme", group: "appearance", kind: SettingValueKind::String, default_value: "Auto", is_secret: false, allowed_values: &["Auto", "Light", "Dark"] },
    SettingSchema { key: "heartbeatBarTheme", group: "appearance", kind: SettingValueKind::String, default_value: "Normal", is_secret: false, allowed_values: &["Normal", "Bottom", "None"] },
    SettingSchema { key: "errorNotificationTimeout", group: "notifications", kind: SettingValueKind::Number, default_value: "20", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "successNotificationTimeout", group: "notifications", kind: SettingValueKind::Number, default_value: "20", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "tlsExpiryNotifyDays", group: "notifications", kind: SettingValueKind::String, default_value: "7,14,21", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "trustedProxies", group: "reverse-proxy", kind: SettingValueKind::String, default_value: "127.0.0.1\n::1", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "tagName", group: "tags", kind: SettingValueKind::String, default_value: "", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "tagColor", group: "tags", kind: SettingValueKind::String, default_value: "green", is_secret: false, allowed_values: &["green", "blue", "red", "yellow"] },
    SettingSchema { key: "keepMonitorHistory", group: "monitor-history", kind: SettingValueKind::Number, default_value: "180", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "dataRetentionEnabled", group: "monitor-history", kind: SettingValueKind::Boolean, default_value: "true", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "dockerDaemon", group: "docker-hosts", kind: SettingValueKind::String, default_value: "/var/run/docker.sock", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "remoteBrowserUrl", group: "remote-browsers", kind: SettingValueKind::String, default_value: "ws://chrome.browserless.io/playwright?token=YOUR-API-TOKEN", is_secret: true, allowed_values: &[] },
    SettingSchema { key: "proxyProtocol", group: "proxies", kind: SettingValueKind::String, default_value: "http", is_secret: false, allowed_values: &["http", "https", "socks", "socks5", "socks5h"] },
    SettingSchema { key: "proxyHost", group: "proxies", kind: SettingValueKind::String, default_value: "", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "proxyPort", group: "proxies", kind: SettingValueKind::Number, default_value: "", is_secret: false, allowed_values: &[] },
    SettingSchema { key: "proxyUsername", group: "proxies", kind: SettingValueKind::String, default_value: "", is_secret: true, allowed_values: &[] },
    SettingSchema { key: "proxyPassword", group: "proxies", kind: SettingValueKind::String, default_value: "", is_secret: true, allowed_values: &[] },
];

pub async fn api_settings() -> Response {
    let handler = "api_settings";
    log_api_start(handler);
    log_api_step(handler, "schema", format!("load schema keys={}", SETTINGS_SCHEMA.len()));
    log_api_step(handler, "config", format!("persistence={}, file={}", active_persistence_mode(), settings_file_path().display()));
    let loaded = load_settings_snapshot(handler).await;
    let settings = loaded.settings;
    log_api_step(handler, "load", format!("load persisted/default settings keys={}", settings.len()));
    log_api_step(handler, "loaded_settings_json", mask_settings_map_for_log(&settings));
    let body = json!({
        "ok": true,
        "settings": settings,
        "meta": {
            "version": 1,
            "source": loaded.source,
            "persistence": loaded.persistence,
            "path": settings_file_path().display().to_string(),
            "groups": settings_groups(),
        }
    });
    let response = api_json_value(handler, body);
    log_api_end(handler);
    response
}

pub async fn api_settings_update(Json(payload): Json<SettingsUpdateRequest>) -> Response {
    let handler = "api_settings_update";
    log_api_start(handler);
    log_api_step(handler, "parse", format!("section={:?}", payload.section));
    log_api_step(handler, "request_json", payload.to_masked_log_json());
    let incoming = normalize_settings_payload(payload);
    log_api_step(handler, "validate", format!("incoming_keys={}", incoming.len()));
    log_api_step(handler, "incoming_json", mask_json_for_log(&Value::Object(incoming.clone()).to_string()));

    let loaded = load_settings_snapshot(handler).await;
    let mut settings = loaded.settings;
    let mut applied_keys = Vec::new();
    let mut rejected = Vec::new();

    for (key, value) in incoming {
        match validate_setting_value(&key, &value) {
            Ok(normalized) => {
                let is_secret = setting_schema(&key).map(|schema| schema.is_secret).unwrap_or(false);
                log_api_step(handler, "apply", format!("key={key}, value={}", if is_secret { "***".to_string() } else { normalized.clone() }));
                settings.insert(key.clone(), normalized);
                applied_keys.push(key);
            }
            Err(reason) => {
                log_api_step(handler, "reject", format!("key={key}, reason={reason}"));
                rejected.push(json!({ "key": key, "reason": reason }));
            }
        }
    }

    let mut persisted = false;
    let mut persist_error = None;
    if rejected.is_empty() {
        log_api_step(handler, "persist.start", format!("keys={}, persistence={}", settings.len(), active_persistence_mode()));
        match save_settings_snapshot(handler, &settings).await {
            Ok(()) => {
                persisted = true;
                log_api_step(handler, "persist.ok", "settings persisted");
            }
            Err(err) => {
                log_api_step(handler, "persist.error", &err);
                persist_error = Some(err);
            }
        }
    } else {
        log_api_step(handler, "persist.skip", format!("rejected_count={}", rejected.len()));
    }

    let ok = rejected.is_empty() && persist_error.is_none();
    let body = json!({
        "ok": ok,
        "appliedKeys": applied_keys,
        "rejected": rejected,
        "settings": settings,
        "meta": {
            "version": 1,
            "source": loaded.source,
            "persistence": active_persistence_mode(),
            "persisted": persisted,
            "persistError": persist_error,
            "path": settings_file_path().display().to_string(),
        }
    });
    let response = api_json_value(handler, body);
    log_api_end(handler);
    response
}

#[derive(Debug)]
struct LoadedSettings {
    settings: BTreeMap<String, String>,
    source: String,
    persistence: String,
}

async fn load_settings_snapshot(handler: &str) -> LoadedSettings {
    let mut settings = default_settings_map();
    if settings_persistence_mode() == SettingsPersistenceMode::Orm {
        match load_settings_from_orm(handler).await {
            Ok(saved) => {
                overlay_valid_settings(&mut settings, saved);
                return LoadedSettings { settings, source: "orm-overlay-schema".to_string(), persistence: "sea-orm".to_string() };
            }
            Err(err) => log_api_step(handler, "repository.orm_fallback", format!("{err}; fallback=file-json")),
        }
    }

    let path = settings_file_path();
    log_api_step(handler, "repository.load", format!("path={}", path.display()));
    if let Ok(content) = fs::read_to_string(&path) {
        match serde_json::from_str::<BTreeMap<String, String>>(&content) {
            Ok(saved) => overlay_valid_settings(&mut settings, saved),
            Err(err) => log_api_step(handler, "repository.load_error", format!("parse failed: {err}")),
        }
    }
    LoadedSettings { settings, source: "file-overlay-schema".to_string(), persistence: active_persistence_mode() }
}

async fn save_settings_snapshot(handler: &str, settings: &BTreeMap<String, String>) -> Result<(), String> {
    if settings_persistence_mode() == SettingsPersistenceMode::Orm {
        match save_settings_to_orm(handler, settings).await {
            Ok(()) => return Ok(()),
            Err(err) => log_api_step(handler, "repository.orm_save_fallback", format!("{err}; fallback=file-json")),
        }
    }

    let path = settings_file_path();
    log_api_step(handler, "repository.save", format!("path={}", path.display()));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("create settings dir failed: {err}"))?;
    }
    let body = serde_json::to_string_pretty(settings).map_err(|err| format!("serialize settings failed: {err}"))?;
    write_atomic(&path, &body).map_err(|err| format!("write settings failed: {err}"))
}

fn write_atomic(path: &Path, body: &str) -> std::io::Result<()> {
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, body)?;
    fs::rename(tmp_path, path)
}

fn settings_file_path() -> PathBuf {
    std::env::var("CORN_SYRUP_SETTINGS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./data/settings.json"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsPersistenceMode {
    File,
    Orm,
}

fn settings_persistence_mode() -> SettingsPersistenceMode {
    match std::env::var("CORN_SYRUP_SETTINGS_PERSISTENCE") {
        Ok(value) if value.eq_ignore_ascii_case("orm") || value.eq_ignore_ascii_case("sea-orm") => SettingsPersistenceMode::Orm,
        _ => SettingsPersistenceMode::File,
    }
}

fn active_persistence_mode() -> String {
    match settings_persistence_mode() {
        SettingsPersistenceMode::File => "file-json".to_string(),
        SettingsPersistenceMode::Orm => "sea-orm-with-file-fallback".to_string(),
    }
}

fn overlay_valid_settings(settings: &mut BTreeMap<String, String>, saved: BTreeMap<String, String>) {
    for (key, value) in saved {
        if setting_schema(&key).is_some() {
            settings.insert(key, value);
        }
    }
}

async fn load_settings_from_orm(handler: &str) -> Result<BTreeMap<String, String>, String> {
    log_api_step(handler, "repository.orm_connect", "load settings via SeaORM");
    let db = connect_settings_orm().await?;
    ensure_settings_table(handler, &db).await?;
    let rows = setting::Entity::find()
        .select_only()
        .column(setting::Column::Key)
        .column(setting::Column::ValueJson)
        .all(&db)
        .await
        .map_err(|err| format!("query settings failed: {err}"))?;
    let mut settings = BTreeMap::new();
    for row in rows {
        let (key, value) = row.to_pair();
        settings.insert(key, value);
    }
    Ok(settings)
}

async fn save_settings_to_orm(handler: &str, settings: &BTreeMap<String, String>) -> Result<(), String> {
    log_api_step(handler, "repository.orm_connect", "save settings via SeaORM");
    let db = connect_settings_orm().await?;
    ensure_settings_table(handler, &db).await?;
    for (key, value) in settings {
        let schema = setting_schema(key).ok_or_else(|| format!("unknown key while saving: {key}"))?;
        let active = setting::SettingUpsert::new(
            key.clone(),
            value.clone(),
            setting_kind_name(schema.kind),
            schema.group,
            schema.is_secret,
        )
        .into_active_model()?;
        setting::Entity::insert(active)
            .on_conflict(
                OnConflict::column(setting::Column::Key)
                    .update_columns([
                        setting::Column::ValueJson,
                        setting::Column::ValueType,
                        setting::Column::GroupName,
                        setting::Column::IsSecret,
                        setting::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(&db)
        .await
        .map_err(|err| format!("upsert setting {key} failed: {err}"))?;
    }
    Ok(())
}

async fn connect_settings_orm() -> Result<sea_orm::DatabaseConnection, String> {
    let url = std::env::var("CORN_SYRUP_SETTINGS_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "sqlite://./data/kuma.db?mode=rwc".to_string());
    sea_orm::Database::connect(&url)
        .await
        .map_err(|err| format!("settings ORM connect failed: {err}"))
}

async fn ensure_settings_table(handler: &str, db: &sea_orm::DatabaseConnection) -> Result<(), String> {
    log_api_step(handler, "repository.orm_bootstrap", "ensure settings table");
    db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        "CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY AUTOINCREMENT, key TEXT NOT NULL UNIQUE, value_json TEXT NOT NULL, value_type TEXT NOT NULL, group_name TEXT NOT NULL, is_secret INTEGER NOT NULL DEFAULT 0, created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)".to_string(),
    ))
    .await
    .map_err(|err| format!("ensure settings table failed: {err}"))?;
    Ok(())
}

fn setting_kind_name(kind: SettingValueKind) -> &'static str {
    match kind {
        SettingValueKind::String => "string",
        SettingValueKind::Number => "number",
        SettingValueKind::Boolean => "boolean",
    }
}

fn api_json_value(handler: &str, body: Value) -> Response {
    let body = body.to_string();
    log_api_step(handler, "prepare_response", "content_type=application/json; charset=utf-8");
    log_api_step(handler, "response_body", mask_sensitive_json_for_log(&body));
    ([("content-type", "application/json; charset=utf-8")], body).into_response()
}

fn default_settings_map() -> BTreeMap<String, String> {
    SETTINGS_SCHEMA.iter().map(|schema| (schema.key.to_string(), schema.default_value.to_string())).collect()
}

fn settings_groups() -> Vec<&'static str> {
    let mut groups = SETTINGS_SCHEMA.iter().map(|schema| schema.group).collect::<Vec<_>>();
    groups.sort();
    groups.dedup();
    groups
}

fn normalize_settings_payload(payload: SettingsUpdateRequest) -> Map<String, Value> {
    let mut data = payload.settings.unwrap_or_default();
    for (key, value) in payload.direct {
        if key != "section" && key != "settings" {
            data.insert(key, value);
        }
    }
    data
}

fn setting_schema(key: &str) -> Option<&'static SettingSchema> {
    SETTINGS_SCHEMA.iter().find(|schema| schema.key == key)
}

fn validate_setting_value(key: &str, value: &Value) -> Result<String, String> {
    let schema = setting_schema(key).ok_or_else(|| "unknown setting key".to_string())?;
    let normalized = match schema.kind {
        SettingValueKind::String => value_to_string(value),
        SettingValueKind::Number => value_to_number_string(value)?,
        SettingValueKind::Boolean => value_to_bool_string(value)?,
    };
    if !schema.allowed_values.is_empty() && !schema.allowed_values.iter().any(|allowed| *allowed == normalized) {
        return Err(format!("invalid value, allowed={}", schema.allowed_values.join("|")));
    }
    Ok(normalized)
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn value_to_number_string(value: &Value) -> Result<String, String> {
    let raw = value_to_string(value);
    if raw.trim().is_empty() {
        return Ok(String::new());
    }
    raw.trim().parse::<i64>().map(|number| number.to_string()).map_err(|_| "expected integer number".to_string())
}

fn value_to_bool_string(value: &Value) -> Result<String, String> {
    match value {
        Value::Bool(flag) => Ok(flag.to_string()),
        Value::String(text) if text.eq_ignore_ascii_case("true") || text == "1" => Ok("true".to_string()),
        Value::String(text) if text.eq_ignore_ascii_case("false") || text == "0" => Ok("false".to_string()),
        _ => Err("expected boolean".to_string()),
    }
}

fn mask_sensitive_json_for_log(body: &str) -> String {
    mask_json_for_log(body)
}

fn mask_json_for_log(body: &str) -> String {
    let mut masked = body.to_string();
    for schema in SETTINGS_SCHEMA.iter().filter(|schema| schema.is_secret) {
        masked = mask_json_key(&masked, schema.key);
    }
    masked
}

fn mask_settings_map_for_log(settings: &BTreeMap<String, String>) -> String {
    let body = serde_json::to_string(settings).unwrap_or_else(|_| "{}".to_string());
    mask_json_for_log(&body)
}

fn mask_json_key(input: &str, key: &str) -> String {
    let pattern = format!("\"{key}\":\"");
    let mut output = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find(&pattern) {
        let (before, after_start) = rest.split_at(start + pattern.len());
        output.push_str(before);
        if let Some(end) = after_start.find('"') {
            output.push_str("***");
            rest = &after_start[end..];
        } else {
            rest = after_start;
            break;
        }
    }
    output.push_str(rest);
    output
}

fn log_api_step(handler: &str, step: &str, detail: impl Into<String>) {
    crate::backend::logging::debug("rest.api.step", &format!("{handler}.{step}"), detail.into());
}

fn log_api_start(handler: &str) {
    log_api_step(handler, "start", "begin call");
}

fn log_api_end(handler: &str) {
    log_api_step(handler, "end", "finish call");
}
