use std::collections::BTreeMap;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct NotificationProviderDescriptor {
    pub id: String,
    pub display_name: String,
    pub supports_test: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NotificationRegistry {
    pub providers: BTreeMap<String, NotificationProviderDescriptor>,
}

impl NotificationRegistry {
    pub fn init() -> Self {
        logging::debug("auto.mod", "init", "enter");
        let mut registry = Self::default();
        for id in [
            "alerta",
            "alertnow",
            "aliyun-sms",
            "apprise",
            "bark",
            "discord",
            "feishu",
            "gotify",
            "line",
            "matrix",
            "mattermost",
            "opsgenie",
            "pushover",
            "rocket-chat",
            "signal",
            "slack",
            "smtp",
            "telegram",
            "webhook",
            "webpush",
            "wecom",
        ] {
            registry.register(id, true);
        }
        registry
    }

    pub fn register(&mut self, id: &str, supports_test: bool) {
        logging::debug("auto.mod", "register", "enter");
        // notification.js 的 Notification.init() 會把 provider class 放入靜態 map；
        // Rust 版以 descriptor registry 保留相同查找與列表輸出行為。
        self.providers.insert(
            id.to_string(),
            NotificationProviderDescriptor {
                id: id.to_string(),
                display_name: titleize(id),
                supports_test,
            },
        );
    }

    pub fn provider_names(&self) -> Vec<String> {
        logging::debug("auto.mod", "provider_names", "enter");
        self.providers.keys().cloned().collect()
    }
}

fn titleize(id: &str) -> String {
    id.split(['-', '_'])
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, Clone, Default)]
pub struct ModContext;

pub fn mod_run(_: &ModContext) -> Result<NotificationRegistry, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    Ok(NotificationRegistry::init())
}
