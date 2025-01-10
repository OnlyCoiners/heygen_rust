use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResponse {
    pub data: Option<VideoData>,
    pub error: Option<ErrorData>,
}
impl VideoResponse {
    pub fn success(video_id: String) -> Self {
        Self {
            data: Some(VideoData { video_id }),
            error: None,
        }
    }

    pub fn error(error: ErrorData) -> Self {
        Self {
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoData {
    pub video_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoDetailsResponse {
    pub code: i32,
    pub data: Option<VideoDetails>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoDetails {
    pub callback_id: Option<String>,
    pub caption_url: Option<String>,
    pub duration: Option<f64>,
    pub error: Option<VideoError>,
    pub gif_url: Option<String>,
    pub id: String,
    pub status: VideoStatus,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub video_url_caption: Option<String>,
    pub created_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoError {
    pub code: i32,
    pub detail: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoStatus {
    Processing,
    Completed,
    Failed,
    Pending,
}

impl VideoDetailsResponse {
    pub fn success(video_details: VideoDetails, code: i32, message: String) -> Self {
        Self {
            data: Some(video_details),
            message: message,
            code: code,
        }
    }

    pub fn error(error: VideoError) -> Self {
        Self {
            code: error.code,
            message: error.message,
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoPayload {
    // pub title: Option<String>,
    // pub caption: Option<bool>,
    pub video_inputs: Vec<VideoInput>,
    pub dimension: Dimension,
}

impl VideoPayload {
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInput {
    pub character: Character,
    pub voice: Voice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "type")]
    pub char_type: String,
    pub avatar_id: String,
    pub avatar_style: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    #[serde(rename = "type")]
    pub voice_type: String,
    pub input_text: String,
    pub voice_id: String,
    pub speed: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub height: u32,
    pub width: u32,
}

impl Dimension {
    /// Returns a dimension with width:1280 and height:720
    pub fn landscape() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }

    /// Returns a dimension with width:720 and height:1280
    pub fn portrait() -> Self {
        Self {
            width: 720,
            height: 1280,
        }
    }
}
