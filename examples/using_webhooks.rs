use axum::{
    body::Bytes, extract::Json, http::StatusCode, response::IntoResponse, routing::post, serve,
    Router,
};
use heygen::schemas::webhook::{WebhookRequest, WebhookResponse};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the server and use smee-client or something like this to expose your endpoint
    // In this example i've used a command like this
    // smee -t http://127.0.0.1:5000/webhook_test -u https://smee.io/YYTcYKjmIOw9
    // This will expose my endopoint to internet forwarding every request sent to the smee to my localhost:5000/webhook_test route

    // When the video get ready, you will receive a post request and will need a handler to print the information from heygen server, this handler can be something like webhook_callback_test
    let app = Router::new().route("/webhook_test", post(webhook_callback_test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn webhook_callback_test(body: Bytes) -> impl IntoResponse {
    let payload: WebhookRequest = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JSON payload").into_response(),
    };

    // After receiving the request, your server should print something like this:
    // Event Type: avatar_video.success; Event Data: {"callback_id":null,"callback_url":null,"folder_id":"","gif_download_url":null,"url":"https://resource2.heygen.ai/video/2541b1cb8c604ec09ae49f47ab36df90/1280x720.mp4?response-content-disposition=attachment%3B+filename%2A%3DUTF-8%27c604ec09ae49f47ab36df90.mp4%3B","video_id":"2541b1cb8c604ec09ae49f47ab36df90","video_share_page_url":null}
    println!(
        "Event Type: {}; Event Data: {}",
        payload.event_type, payload.event_data
    );

    let response = WebhookResponse {
        message: "Success!".to_string(),
    };

    (StatusCode::OK, Json(response)).into_response()
}

// test your server with this if you need
// curl -X POST http://localhost:5000/webhook_test ^
//   -H "Content-Type: application/json" ^
//   -H "Signature: whsec_6TMIemLohuOIjww==" ^
//   -d "{\"event_type\": \"test_event\", \"event_data\": {\"key\": \"value\"}}"
