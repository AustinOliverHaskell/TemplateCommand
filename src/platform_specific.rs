#[cfg(windows)]
pub const PLATFORM_SEPARATOR_SLASH: &str = "\\";

#[cfg(unix)]
pub const PLATFORM_SEPARATOR_SLASH: &str = "/";