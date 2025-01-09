use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListVideosResponse {
    pub code: i32,
    pub data: Option<VideoListData>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoListData {
    pub token: Option<String>,
    pub videos: Vec<Video>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub video_id: String,
    pub video_title: String,
    pub status: VideoStatus,
    pub created_at: i64,
    #[serde(rename = "type")]
    pub video_type: VideoType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoStatus {
    Completed,
    Processing,
    Failed,
    Draft,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VideoType {
    Generated,
    Translated,
}

impl ListVideosResponse {
    pub fn error(error: ListErrorData) -> Self {
        Self {
            message: Some(error.message),
            code: error.code,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListErrorData {
    pub code: i32,
    pub message: String,
}
