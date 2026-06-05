use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>, // epoch ms
}

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>, // seconds
}

#[derive(Deserialize)]
pub struct FilesPage {
    #[serde(default)]
    pub data: Vec<FileSummary>,
    #[serde(default)]
    pub page: i64,
}

#[derive(Deserialize)]
pub struct FileSummary {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub serial_number: Option<String>,
}

#[derive(Deserialize)]
pub struct FileDetail {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub start_at: Option<String>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub serial_number: Option<String>,
    #[serde(default)]
    pub presigned_url: Option<String>,
    #[serde(default)]
    pub source_list: Vec<DataItem>,
    #[serde(default)]
    pub note_list: Vec<DataItem>,
}

#[derive(Deserialize)]
pub struct DataItem {
    #[serde(default)]
    pub data_type: String,
    #[serde(default)]
    pub data_content: String,
}

#[derive(Deserialize)]
pub struct Segment {
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub speaker: Option<String>,
    #[serde(default)]
    pub content: String,
}
