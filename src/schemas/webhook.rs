use serde::Serialize;
use url::Url;

// TODO
// Use Url typeo or others for url instead of url
#[derive(Serialize)]
pub struct RegisterWebhookRequest {
    pub url: Url,
    pub events: Vec<String>,
}

#[derive(Serialize)]
pub struct UpdateWebhookRequest {
    pub url: Url,
    pub events: Vec<String>,
    pub endpoint_id: String,
}
