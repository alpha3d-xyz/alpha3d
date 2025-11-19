use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct GreetingResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EchoPayload {
    message: String,
}

#[tokio::test]
async fn test_get_greeting() -> Result<()> {
    let base_url = env::var("TEST_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/api/greeting", base_url))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: GreetingResponse = resp.json().await?;
    assert_eq!(body.message, "Hello from Axum + Vue template");

    Ok(())
}

#[tokio::test]
async fn test_post_echo() -> Result<()> {
    let base_url = env::var("TEST_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let client = reqwest::Client::new();

    let payload = EchoPayload {
        message: "Acceptance Test".to_string(),
    };

    let resp = client
        .post(format!("{}/api/echo", base_url))
        .json(&payload)
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: EchoPayload = resp.json().await?;
    assert_eq!(body, payload);

    Ok(())
}
