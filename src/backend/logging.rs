use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_debug_enabled(enabled: bool) {
    DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn debug(scope: &str, action: &str, detail: impl AsRef<str>) {
    if DEBUG_ENABLED.load(Ordering::Relaxed) {
        eprintln!("[DEBUG][corn-syrup][{scope}] {action}: {}", detail.as_ref());
    }
}

pub fn info(scope: &str, action: &str, detail: impl AsRef<str>) {
    eprintln!("[INFO][corn-syrup][{scope}] {action}: {}", detail.as_ref());
}

pub fn warn(scope: &str, action: &str, detail: impl AsRef<str>) {
    eprintln!("[WARN][corn-syrup][{scope}] {action}: {}", detail.as_ref());
}
