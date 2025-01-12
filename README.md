# Heygen

`heygen` is a Rust SDK for interacting with the HeyGen API, providing an ergonomic and type-safe way to create and manage AI-generated videos.

[![Crates.io](https://img.shields.io/crates/v/heygen)](https://crates.io/crates/heygen)
[![Documentation](https://docs.rs/heygen/badge.svg)](https://docs.rs/heygen)

### Help us by using our affiliate link!

Join Heygen using [this link](https://heygen.com/?sid=rewardful&via=onlycoiners) and you will be rewarding our work! Thank you!

## High level features

- Create AI-generated avatar videos with comprehensive configuration options
- Manage templates for premium HeyGen creators
- Handle webhook registrations and notifications
- List and retrieve video details and status
- Fully typed responses and error handling
- Built on top of `reqwest` for reliable HTTP communication

## Usage example

```rust
use heygen::HeyGenBot;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Create a new HeyGen client
    let client = HeyGenBot::new("your-api-key".to_string(), Some("https://api.heygen.com/v2/")).unwrap();

    // Create a video using an avatar
    let payload = json!({
        "video_inputs": [{
            "character": {
                "type": "avatar",
                "avatar_id": "Angela-inTshirt-20220820",
                "avatar_style": "normal"
            },
            "voice": {
                "type": "text",
                "input_text": "Hello from the HeyGen Rust SDK!",
                "voice_id": "1bd001e7e50f421d891986aad5158bc8",
                "speed": 1.1
            }
        }],
        "dimension": {
            "width": 1280,
            "height": 720
        }
    });

    // Generate the video
    match client.create_avatar_video(payload).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("Video generated successfully! ID: {}", data.video_id);
            }
        }
        Err(e) => eprintln!("Error generating video: {}", e),
    }
}
```

## Features

### Video Generation

- Create avatar videos with customizable parameters
- Generate videos from templates (premium feature)
- Monitor video generation status
- List all generated videos

### Template Management

- List available templates
- Retrieve template details
- Generate videos from existing templates

### Webhook Management

- Register webhook endpoints
- List registered webhooks
- Update webhook configurations
- Delete webhook registrations

### Avatar Management

- List all available avatars

## Examples

The [examples](examples/) directory contains various examples showcasing different features:

To run an example use cargo run --example <example_file_name>

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
heygen = "0.1.2"
```

## Documentation

For detailed documentation and API reference, check out [heygen API](https://docs.heygen.com/reference/authentication).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT license.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `heygen` by you shall be licensed as MIT, without any additional
terms or conditions.
