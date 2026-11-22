#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> { goodstock_desktop::desktop_main() }
#[cfg(target_os = "android")]
fn main() {}
