#[cfg(windows)] #[allow(dead_code)]
pub const PLATFORM_SEPARATOR_SLASH: &str = "\\";

#[cfg(unix)] #[allow(dead_code)]
pub const PLATFORM_SEPARATOR_SLASH: &str = "/";



#[cfg(unix)] #[allow(dead_code)]
pub const PLATFORM_LINE_ENDING: &str = "\n";

#[cfg(windows)] #[allow(dead_code)]
pub const PLATFORM_LINE_ENDING: &str = "\r\n";