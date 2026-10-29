// Allow the crate to have a non-snake-case name (touchHLE).
// This also allows items in the crate to have non-snake-case names.
#![allow(non_snake_case)]

/// Current version. See `build.rs` for how this is generated.
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/version.txt"));

/// UI-facing version for the iDared fork
pub const DISPLAY_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/display_version.txt"));

// Environment variables set by GitHub Actions
pub const GITHUB_REPOSITORY: Option<&str> = option_env!("GITHUB_REPOSITORY");
pub const GITHUB_SERVER_URL: Option<&str> = option_env!("GITHUB_SERVER_URL");
pub const GITHUB_RUN_ID: Option<&str> = option_env!("GITHUB_RUN_ID");
pub const GITHUB_REF_NAME: Option<&str> = option_env!("GITHUB_REF_NAME");
pub const GITHUB_EVENT_NAME: Option<&str> = option_env!("GITHUB_EVENT_NAME");

pub fn branding() -> &'static str {
    return "";
}
