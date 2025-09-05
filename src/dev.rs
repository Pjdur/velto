use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static DEV_MODE: OnceLock<AtomicBool> = OnceLock::new();

pub fn set_dev_mode(enabled: bool) {
    DEV_MODE
        .get_or_init(|| AtomicBool::new(enabled))
        .store(enabled, Ordering::Relaxed);
}

pub fn is_dev_mode() -> bool {
    DEV_MODE
        .get()
        .map(|f| f.load(Ordering::Relaxed))
        .unwrap_or(false)
}
