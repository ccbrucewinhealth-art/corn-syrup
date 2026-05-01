use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct SettingValue {
    pub value: String,
    pub setting_type: Option<String>,
    pub timestamp: SystemTime,
}

pub trait SettingsStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: String);
    fn set_typed(&self, key: &str, value: String, setting_type: Option<String>);
    fn get_settings_by_prefix(&self, prefix: &str) -> HashMap<String, String>;
    fn get_settings(&self, setting_type: &str) -> HashMap<String, String>;
    fn delete_cache(&self, keys: &[String]);
    fn clear_expired_cache(&self, max_age: Duration);
}

#[derive(Debug, Clone, Default)]
pub struct InMemorySettingsStore {
    cache: Arc<Mutex<HashMap<String, SettingValue>>>,
}

impl SettingsStore for InMemorySettingsStore {
    fn get(&self, key: &str) -> Option<String> {
        logging::debug("settings", "get", key);
        // 對齊 settings.js get()：先查 cacheList，命中後直接回傳 value。
        self.cache
            .lock()
            .ok()
            .and_then(|m| m.get(key).map(|v| v.value.clone()))
    }

    fn set(&self, key: &str, value: String) {
        logging::debug("settings", "set", key);
        self.set_typed(key, value, None);
    }

    fn set_typed(&self, key: &str, value: String, setting_type: Option<String>) {
        logging::debug("settings", "set_typed", key);
        if let Ok(mut m) = self.cache.lock() {
            // settings.js set() 會 JSON.stringify 後存入資料庫並刪除 cache；
            // 目前 Rust 記憶體版以最新值覆蓋 cache，保留 type/timestamp 以支援 getSettings 與過期清理。
            m.insert(
                key.to_string(),
                SettingValue {
                    value,
                    setting_type,
                    timestamp: SystemTime::now(),
                },
            );
        }
    }

    fn get_settings_by_prefix(&self, prefix: &str) -> HashMap<String, String> {
        logging::debug("settings", "get_settings_by_prefix", prefix);
        if let Ok(m) = self.cache.lock() {
            m.iter()
                .filter(|(k, _)| k.starts_with(prefix))
                .map(|(k, v)| (k.clone(), v.value.clone()))
                .collect()
        } else {
            HashMap::new()
        }
    }

    fn get_settings(&self, setting_type: &str) -> HashMap<String, String> {
        logging::debug("settings", "get_settings", setting_type);
        if let Ok(m) = self.cache.lock() {
            m.iter()
                .filter(|(_, v)| v.setting_type.as_deref() == Some(setting_type))
                .map(|(k, v)| (k.clone(), v.value.clone()))
                .collect()
        } else {
            HashMap::new()
        }
    }

    fn delete_cache(&self, keys: &[String]) {
        logging::debug("settings", "delete_cache", format!("keys={}", keys.len()));
        if let Ok(mut m) = self.cache.lock() {
            for key in keys {
                m.remove(key);
            }
        }
    }

    fn clear_expired_cache(&self, max_age: Duration) {
        logging::debug("settings", "clear_expired_cache", format!("max_age_secs={}", max_age.as_secs()));
        if let Ok(mut m) = self.cache.lock() {
            let now = SystemTime::now();
            // settings.js cacheCleaner 每分鐘刪除超過 60 秒的項目；
            // 這裡以呼叫端給定 max_age 執行同一個「timestamp 差值大於門檻即移除」演算法。
            m.retain(|_, v| now.duration_since(v.timestamp).unwrap_or_default() <= max_age);
        }
    }
}
