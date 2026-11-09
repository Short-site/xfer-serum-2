//! Static build info for Xfer Serum 2.

pub const BUILD_VERSION: &str  = "v4.5.23";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "a319adade437";
pub const BUILD_TIMESTAMP: u64 = 1712846968;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Xfer Serum 2", BUILD_VERSION, BUILD_CHANNEL)
}
