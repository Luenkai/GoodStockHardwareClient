#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> { goodstock_desktop::desktop_main() }
#[cfg(target_os = "android")]
fn main() {}
