mod client;
mod fmt;
mod oauth;
mod types;

pub use client::Client;
pub use fmt::{
    format_date, format_duration, format_time, segments_to_srt, segments_to_text,
    summary_markdown, transcript_segments,
};
pub use oauth::{run_login, OAuth, TokenStore};
pub use types::{DataItem, FileDetail, FileSummary, FilesPage, Segment, TokenSet};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Public OAuth client shipped by Plaud's official CLI (public client + PKCE).
pub const CLIENT_ID: &str = "client_f9e0b214-c11f-434b-8b95-c4497d1feb81";
pub const REDIRECT_URI: &str = "http://localhost:8199/auth/callback";
pub const CALLBACK_PORT: u16 = 8199;
pub const AUTH_URL: &str = "https://web.plaud.ai/platform/oauth";
pub const TOKEN_URL: &str =
    "https://platform.plaud.ai/developer/api/oauth/third-party/access-token";
pub const REFRESH_URL: &str =
    "https://platform.plaud.ai/developer/api/oauth/third-party/access-token/refresh";
pub const API_BASE: &str = "https://platform.plaud.ai/developer/api";