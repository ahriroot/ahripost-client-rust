#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ahripost_client_rust::entity;
use serde_json::{json, Value};
use tauri::Manager;

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn get(request: entity::Request) -> Value {
    let href = format!("{}//{}:{}", request.protocol, request.host, request.port);
    println!("href: {}", href);
    let response_result = reqwest::get(href).await;
    match response_result {
        Ok(response) => {
            // 遍历 headers
            let mut headers = json!({});
            for (key, value) in response.headers() {
                headers[key.as_str()] = json!(value.to_str().unwrap());
            }
            let version = match response.version() {
                reqwest::Version::HTTP_09 => "HTTP/0.9",
                reqwest::Version::HTTP_10 => "HTTP/1.0",
                reqwest::Version::HTTP_11 => "HTTP/1.1",
                reqwest::Version::HTTP_2 => "HTTP/2.0",
                reqwest::Version::HTTP_3 => "HTTP/3.0",
                _ => "Unknown",
            };
            json!({
                "status": response.status().as_u16(),
                "canonical_reason": response.status().canonical_reason().unwrap_or("Unknown"),
                "version": version,
                "headers": headers,
                "body": response.text().await.unwrap()
            })
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();

    app = app.invoke_handler(tauri::generate_handler![close_splashscreen, get]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
