use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs;
use reqwest::multipart;

#[derive(Debug, Serialize, Deserialize)]
struct GreetingResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EchoPayload {
    message: String,
}

#[derive(Serialize)]
struct SignupPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UploadResponse {
    file_id: uuid::Uuid,
    filename: String,
    volume_cm3: f64,
    surface_area_cm2: f64,
}

async fn get_auth_token(base_url: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let email = format!("test_{}@example.com", uuid::Uuid::new_v4());
    let password = "password123";

    // Signup
    let _ = client.post(format!("{}/api/auth/signup", base_url))
        .json(&SignupPayload {
            username: "testuser".to_string(),
            email: email.clone(),
            password: password.to_string(),
        })
        .send()
        .await?;

    // Login
    let resp = client.post(format!("{}/api/auth/login", base_url))
        .json(&LoginPayload {
            email,
            password: password.to_string(),
        })
        .send()
        .await?;

    let body: AuthResponse = resp.json().await?;
    Ok(body.token)
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

#[tokio::test]
async fn test_upload_all_stl_files_sorted() -> Result<()> {
    let base_url = env::var("TEST_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let token = get_auth_token(&base_url).await?;
    let client = reqwest::Client::new();

    let mut entries = Vec::new();
    let mut dir = fs::read_dir("data/stl").await?;
    while let Some(entry) = dir.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            entries.push((entry, metadata.len()));
        }
    }

    // Sort by file size
    entries.sort_by_key(|k| k.1);

    for (entry, size) in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        println!("Uploading {} ({} bytes)", filename, size);

        let file_content = fs::read(&path).await?;
        let part = multipart::Part::bytes(file_content).file_name(filename.clone());
        let form = multipart::Form::new().part("file", part);

        let resp = client.post(format!("{}/api/files/upload", base_url))
            .header("Authorization", format!("Bearer {}", token))
            .multipart(form)
            .send()
            .await?;

        if size > 100 * 1024 * 1024 {
            assert_eq!(resp.status(), 413, "Expected 413 for file {} of size {}", filename, size);
        } else {
            let status = resp.status();
            if status != 201 {
                let error_text = resp.text().await.unwrap_or_else(|_| "Could not read error".to_string());
                panic!("Expected 201 for file {} but got {}. Error: {}", filename, status, error_text);
            }
            let body: UploadResponse = resp.json().await?;
            println!("Analyzed {}: Volume={:.2} cm3, Surface Area={:.2} cm2", filename, body.volume_cm3, body.surface_area_cm2);
            
            assert!(body.volume_cm3 >= 0.0, "Volume should be non-negative");
            assert!(body.surface_area_cm2 >= 0.0, "Surface area should be non-negative");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_upload_large_file() -> Result<()> {
    let base_url = env::var("TEST_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let token = get_auth_token(&base_url).await?;
    let client = reqwest::Client::new();

    // Create a dummy large file content > 100MB
    let large_content = vec![0u8; 101 * 1024 * 1024];
    let part = multipart::Part::bytes(large_content).file_name("large.stl");
    let form = multipart::Form::new().part("file", part);

    let resp = client.post(format!("{}/api/files/upload", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await?;

    assert_eq!(resp.status(), 413);
    Ok(())
}
