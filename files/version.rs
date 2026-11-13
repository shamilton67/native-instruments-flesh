//! Static build info for Native Instruments Flesh.

pub const BUILD_VERSION: &str  = "v1.6.24";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "842436f14065";
pub const BUILD_TIMESTAMP: u64 = 1785614933;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Native Instruments Flesh", BUILD_VERSION, BUILD_CHANNEL)
}
