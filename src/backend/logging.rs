use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(true);
static INIT_LOGGER: Once = Once::new();

pub fn init(level: &str) {
    let normalized_level = match level.trim().to_ascii_lowercase().as_str() {
        "trace" => "trace",
        "debug" => "debug",
        "info" => "info",
        "warn" | "warning" => "warn",
        "error" => "error",
        "off" => "off",
        _ => "debug",
    };

    set_debug_enabled(matches!(normalized_level, "trace" | "debug"));
    INIT_LOGGER.call_once(|| {
        let mut builder = env_logger::Builder::new();
        builder.parse_filters(normalized_level);
        builder.format_timestamp_secs();
        let _ = builder.try_init();
    });
}

pub fn set_debug_enabled(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn debug(scope: &str, action: &str, detail: impl AsRef<str>) {
    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        log::debug!(target: "corn_syrup", "[{scope}] {action}: {}", detail.as_ref());
    }
}

pub fn info(scope: &str, action: &str, detail: impl AsRef<str>) {
    log::info!(target: "corn_syrup", "[{scope}] {action}: {}", detail.as_ref());
}

pub fn warn(scope: &str, action: &str, detail: impl AsRef<str>) {
    log::warn!(target: "corn_syrup", "[{scope}] {action}: {}", detail.as_ref());
}
