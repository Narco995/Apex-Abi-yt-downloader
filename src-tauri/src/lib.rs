// lib.rs — mobile entry point (unused on desktop; all setup is in main.rs)
// Kept minimal to avoid dead-code warnings from the default tauri init stub.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Mobile entry point — not called on desktop.
    // If you add mobile support, wire up plugins and commands here.
}
