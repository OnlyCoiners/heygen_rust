use std::collections::HashMap;

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
pub struct TemplateErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTemplatesResponse {
    pub data: Option<TemplateData>,
    pub error: Option<TemplateErrorData>,
}

impl ListTemplatesResponse {
    pub fn success(templates: Vec<Template>) -> Self {
        Self {
            data: Some(TemplateData { templates }),
            error: None,
        }
    }

    pub fn error(error: TemplateErrorData) -> Self {
        Self {
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateDetailsResponse {
    pub data: Option<TemplateDetailsData>,
    pub error: Option<TemplateError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateDetailsData {
    pub variables: HashMap<String, VariableDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDetails {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: String,
    pub properties: VariableProperties,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VariableProperties {
    Text {
        content: String,
    },
    Image {
        url: String,
        asset_id: Option<String>,
        fit: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateError {
    pub code: String,
    pub message: String,
}

impl TemplateDetailsResponse {
    pub fn success(template: TemplateDetailsData) -> Self {
        Self {
            data: Some(template),
            error: None,
        }
    }

    pub fn error(error: TemplateError) -> Self {
        Self {
            data: None,
            error: Some(error),
        }
    }
}
