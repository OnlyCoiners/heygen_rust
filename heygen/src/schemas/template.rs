use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub template_id: String,
    pub thumbnail_image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
    pub templates: Vec<Template>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTemplatesResponse {
    pub data: Option<TemplateData>,
    pub error: Option<ErrorData>,
}
