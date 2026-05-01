use crate::backend::settings::SettingsStore;
use crate::backend::logging;

pub const UPDATE_CHECKER_INTERVAL_MS: u64 = 1000 * 60 * 60 * 48;
pub const UPDATE_CHECKER_LATEST_VERSION_URL: &str = "https://uptime.kuma.pet/version";

#[derive(Debug, Clone)]
pub struct VersionPayload {
    pub slow: Option<String>,
    pub beta: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VersionChecker {
    pub version: String,
    pub latest_version: Option<String>,
    pub interval_ms: u64,
    pub url: String,
    pub interval_running: bool,
}

impl VersionChecker {
    pub fn new(version: impl Into<String>) -> Self {
        logging::debug("auto.version_check", "new", "enter");
        Self {
            version: version.into(),
            latest_version: None,
            interval_ms: UPDATE_CHECKER_INTERVAL_MS,
            url: UPDATE_CHECKER_LATEST_VERSION_URL.to_string(),
            interval_running: false,
        }
    }

    pub fn start_interval<S: SettingsStore>(&mut self, store: &S, payload: VersionPayload) {
        self.interval_running = true;
        self.check_once(store, payload);
    }

    pub fn check_once<S: SettingsStore>(&mut self, store: &S, payload: VersionPayload) {
        if store.get("checkUpdate").as_deref() == Some("false") {
            return;
        }

        let check_beta = store.get("checkBeta").as_deref() == Some("true");
        if check_beta {
            if let (Some(beta), Some(slow)) = (&payload.beta, &payload.slow) {
                if compare_semver(beta, slow).is_gt() {
                    self.latest_version = Some(beta.clone());
                    return;
                }
            }
        }

        // 對齊 check-version.js：beta 未啟用或未高於 slow 時，以 slow 作為 latestVersion。
        if let Some(slow) = payload.slow {
            self.latest_version = Some(slow);
        }
    }

    pub fn enable_check_update<S: SettingsStore>(
        &mut self,
        store: &S,
        value: bool,
        payload: VersionPayload,
    ) {
        store.set("checkUpdate", value.to_string());
        self.interval_running = false;
        if value {
            self.start_interval(store, payload);
        }
    }
}

pub fn compare_semver(left: &str, right: &str) -> std::cmp::Ordering {
    logging::debug("auto.version_check", "compare_semver", "enter");
    let parse = |v: &str| -> Vec<u64> {
        v.split('.')
            .take(3)
            .map(|part| {
                part.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
            })
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let mut l = parse(left);
    let mut r = parse(right);
    l.resize(3, 0);
    r.resize(3, 0);
    l.cmp(&r)
}

#[derive(Debug, Clone)]
pub struct VersionCheckContext {
    pub version: String,
}

pub fn version_check_run(ctx: &VersionCheckContext) -> Result<VersionChecker, String> {
    logging::debug("auto.version_check", "version_check_run", "enter");
    if ctx.version.trim().is_empty() {
        return Err("version is required".to_string());
    }
    Ok(VersionChecker::new(ctx.version.clone()))
}
