use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static DEV_MODE: OnceLock<AtomicBool> = OnceLock::new();
static RELOAD_PORT: OnceLock<u16> = OnceLock::new();

/// Enables or disables development mode globally.
///
/// Development mode activates features like LiveReload and verbose logging.
/// This should be called early in your app setup via `App::enable_dev_mode()`.
///
/// # Arguments
///
/// * `enabled` - `true` to enable dev mode, `false` to disable.
pub fn set_dev_mode(enabled: bool) {
    DEV_MODE
        .get_or_init(|| AtomicBool::new(enabled))
        .store(enabled, Ordering::Relaxed);
}

/// Returns whether development mode is currently enabled.
///
/// This is used internally to toggle features like LiveReload injection.
pub fn is_dev_mode() -> bool {
    DEV_MODE
        .get()
        .map(|f| f.load(Ordering::Relaxed))
        .unwrap_or(false)
}

/// Sets the port used by the LiveReload WebSocket server.
///
/// This is called internally by the reload system when a free port is found.
/// Templates use this value to inject the correct WebSocket address.
///
/// # Arguments
///
/// * `port` - The port number to assign for LiveReload.
pub fn set_reload_port(port: u16) {
    RELOAD_PORT.set(port).ok();
}

/// Returns the current LiveReload WebSocket port.
///
/// Defaults to `35729` if no port has been set.
pub fn get_reload_port() -> u16 {
    *RELOAD_PORT.get().unwrap_or(&35729)
}
