use anyhow::Result;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde_json::Value;
use std::error::Error;
use url::{ParseError, Url};

use crate::schemas::{
    list_videos::{ListErrorData, ListVideosResponse},
    template::{ListTemplatesResponse, TemplateDetailsResponse, TemplateError, TemplateErrorData},
    video::{ErrorData, VideoDetailsResponse, VideoError, VideoResponse},
    webhook::{RegisterWebhookRequest, UpdateWebhookRequest},
};

pub struct HeyGenBot {
    api_key: String,
    base_url: Url,
    client: Client,
}

impl HeyGenBot {
    /// Creates a new HeyGenBot instance with an API key and a base URL.
    pub fn new(api_key: String, base_url: Option<&str>) -> Result<Self, ParseError> {
        let base_url = Url::parse(base_url.unwrap_or("https://api.heygen.com/v1/"))?;
        Ok(Self {
            api_key,
            base_url,
            client: Client::new(),
        })
    }

    /// Helper method to create a RequestBuilder with the base URL and a specific path.
    fn build_request(&self, method: Method, path: &str) -> Result<RequestBuilder, ParseError> {
        let url = self.base_url.join(path)?;
        Ok(self
            .client
            .request(method, url.as_str())
            .header("X-Api-Key", &self.api_key)
            .header("Accept", "application/json"))
    }

    pub async fn register_webhook(
        &self,
        endpoint_url: &str,
        events: Vec<String>,
    ) -> Result<Value, Box<dyn Error>> {
        let payload = RegisterWebhookRequest {
            url: Url::parse(endpoint_url)?,
            events: events.into_iter().map(String::from).collect(),
        };
        let response = self
            .build_request(Method::POST, "endpoint.add")?
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(response)
    }

    pub async fn list_webhooks(&self) -> Result<Value> {
        let response = self
            .build_request(Method::GET, "endpoint.list")?
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    pub async fn update_webhook(
        &self,
        endpoint_id: &str,
        new_url: &str,
        events: Vec<String>,
    ) -> Result<Value, Box<dyn Error>> {
        let payload = UpdateWebhookRequest {
            endpoint_id: endpoint_id.to_string(),
            url: Url::parse(new_url)?,
            events: events.into_iter().map(String::from).collect(),
        };

        let response = self
            .build_request(Method::PATCH, "endpoint.update")?
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    pub async fn delete_webhook(&self, endpoint_id: &str) -> Result<Value, Box<dyn Error>> {
        let path = format!("endpoint.delete?endpoint_id={}", endpoint_id);
        let response = self
            .build_request(Method::DELETE, &path)?
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response)
    }

    /// Retrieves all avatars.
    pub async fn list_all_avatars(&self) -> Result<Value, Box<dyn Error>> {
        let response = self.build_request(Method::GET, "avatars")?.send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(format!("HTTP Error {}: {}", status, body).into());
        }
        let json: Value = serde_json::from_str(&body)?;

        Ok(json)
    }

    pub async fn create_avatar_video(
        &self,
        payload: Value,
    ) -> Result<VideoResponse, Box<dyn Error>> {
        let response = self
            .build_request(Method::POST, "video/generate")?
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(VideoResponse::error(ErrorData {
                    message: format!("{} {}", status, body),
                    code: StatusCode::NOT_FOUND.as_str().to_string(),
                }));
            }
        }
        let video_response: VideoResponse = serde_json::from_str(&body)?;

        Ok(video_response)
    }

    /// Retrieves specific video details.
    pub async fn retrieve_video_details(
        &self,
        video_id: &str,
    ) -> Result<VideoDetailsResponse, Box<dyn Error>> {
        let path = format!("video_status.get?video_id={}", video_id);

        let response = self.build_request(Method::GET, &path)?.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(VideoDetailsResponse::error(VideoError {
                    message: format!("{} {}", status, body),
                    code: 404,
                    detail: Some("Not found".to_string()),
                }));
            }
        }
        let video_response: VideoDetailsResponse = serde_json::from_str(&body)?;

        Ok(video_response)
    }

    /// Retrieves all templates.
    pub async fn list_templates(&self) -> Result<ListTemplatesResponse, Box<dyn Error>> {
        let response = self.build_request(Method::GET, "templates")?.send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(ListTemplatesResponse::error(TemplateErrorData {
                    message: format!("{} {}", status, body),
                    code: StatusCode::NOT_FOUND.as_str().to_string(),
                }));
            }
        }

        let template_response: ListTemplatesResponse = serde_json::from_str(&body)?;

        Ok(template_response)
    }

    /// Retrieves specific template details.
    pub async fn retrieve_template_details(
        &self,
        template_id: &str,
    ) -> Result<TemplateDetailsResponse, Box<dyn Error>> {
        let path = format!("template/{}", template_id);

        let response = self.build_request(Method::GET, &path)?.send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(TemplateDetailsResponse::error(TemplateError {
                    message: format!("{} {}", status, body),
                    code: "404".to_string(),
                }));
            }
        }
        let template_response: TemplateDetailsResponse = serde_json::from_str(&body)?;

        Ok(template_response)
    }

    // Retrieves specific template details.
    pub async fn generate_video_from_template(
        &self,
        template_id: &str,
        payload: Value,
    ) -> Result<VideoResponse, Box<dyn Error>> {
        let path = format!("template/{}/generate", template_id);

        let response = self
            .build_request(Method::POST, &path)?
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(VideoResponse::error(ErrorData {
                    message: format!("{} {}", status, body),
                    code: StatusCode::NOT_FOUND.as_str().to_string(),
                }));
            }
        }
        let video_response: VideoResponse = serde_json::from_str(&body)?;

        Ok(video_response)
    }

    /// Lists videos with optional limit
    pub async fn list_videos(&self, limit: u32) -> Result<ListVideosResponse, Box<dyn Error>> {
        let path = format!("video.list?limit={}", limit);

        let response = self.build_request(Method::GET, &path)?.send().await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if status.as_u16() == StatusCode::NOT_FOUND.as_u16() && !body.contains("internal_error")
            {
                return Ok(ListVideosResponse::error(ListErrorData {
                    message: format!("{} {}", status, body),
                    code: 404,
                }));
            }
        }
        let video_response: ListVideosResponse = serde_json::from_str(&body)?;

        Ok(video_response)
    }
}
