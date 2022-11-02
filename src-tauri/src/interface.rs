use crate::entity;
use reqwest;
use serde_json::{json, Value};

#[tauri::command]
pub async fn sync_api(data: Value, server: String, token: String) -> Value {
    let href = format!("{}/client/api/sync", server);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes("application/json".as_bytes()).unwrap(),
    );
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Authorization".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap(),
    );
    // let json_data: Value = serde_json::from_str("{}").unwrap();
    let response_result = reqwest::Client::new()
        .request(reqwest::Method::POST, href.as_str())
        .headers(headers)
        .json(&data)
        .send()
        .await;
    match response_result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let res: Value = text.parse().unwrap();
            res
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

#[tauri::command]
pub async fn sync_check(data: Value, server: String, token: String) -> Value {
    let href = format!("{}/client/api/sync_check", server);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes("application/json".as_bytes()).unwrap(),
    );
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Authorization".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap(),
    );
    // let json_data: Value = serde_json::from_str("{}").unwrap();
    let response_result = reqwest::Client::new()
        .request(reqwest::Method::POST, href.as_str())
        .headers(headers)
        .json(&data)
        .send()
        .await;
    match response_result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let res: Value = text.parse().unwrap();
            res
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

#[tauri::command]
pub async fn sync_data(data: Value, server: String, token: String) -> Value {
    let href = format!("{}/client/api/sync_data", server);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes("application/json".as_bytes()).unwrap(),
    );
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Authorization".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap(),
    );
    // let json_data: Value = serde_json::from_str("{}").unwrap();
    let response_result = reqwest::Client::new()
        .request(reqwest::Method::POST, href.as_str())
        .headers(headers)
        .json(&data)
        .send()
        .await;
    match response_result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let res: Value = text.parse().unwrap();
            res
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

#[tauri::command]
pub async fn load_project(server: String, token: String) -> Value {
    let href = format!("{}/client/api/project", server);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes("application/json".as_bytes()).unwrap(),
    );
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Authorization".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap(),
    );
    // let json_data: Value = serde_json::from_str("{}").unwrap();
    let response_result = reqwest::Client::new()
        .request(reqwest::Method::GET, href.as_str())
        .headers(headers)
        .send()
        .await;
    match response_result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let res: Value = text.parse().unwrap();
            res
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

#[tauri::command]
pub async fn download_project(server: String, token: String) -> Value {
    let href = format!("{}/client/api/sync", server);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Content-Type".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes("application/json".as_bytes()).unwrap(),
    );
    headers.insert(
        reqwest::header::HeaderName::from_bytes("Authorization".as_bytes()).unwrap(),
        reqwest::header::HeaderValue::from_bytes(token.as_bytes()).unwrap(),
    );
    // let json_data: Value = serde_json::from_str("{}").unwrap();
    let response_result = reqwest::Client::new()
        .request(reqwest::Method::POST, href.as_str())
        .headers(headers)
        .send()
        .await;
    match response_result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            let res: Value = text.parse().unwrap();
            res
        }
        Err(err) => json!({ "error": err.to_string() }),
    }
}

#[tauri::command]
pub async fn start_login_server() -> bool {
    println!("start_login_server start");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(12);

    tokio::spawn(async move {
        // let listener = tokio::net::TcpListener::bind("127.0.0.1").await.unwrap();
        // let port = listener.local_addr().unwrap().port();

        // for stream in listener.incoming() {
        //     let stream = stream.unwrap();
        //     thread::spawn(|| handle_connection(stream));
        // }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        tx.send(0).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        println!("start_login_server finish");
    });
    if rx.recv().await.is_some() {
        return true;
    }
    false
}

#[tauri::command]
pub async fn request(request: entity::Request) -> Value {
    let href = format!(
        "{}//{}:{}{}",
        request.protocol, request.host, request.port, request.path
    );
    let params = request
        .params
        .iter()
        .map(|param| (param.field.as_str(), param.value.as_str()))
        .collect::<Vec<(&str, &str)>>();
    let mut headers = reqwest::header::HeaderMap::new();
    for header in request.headers {
        headers.insert(
            reqwest::header::HeaderName::from_bytes(header.field.as_bytes()).unwrap(),
            reqwest::header::HeaderValue::from_bytes(header.value.as_bytes()).unwrap(),
        );
    }
    // let body: reqwest::Body = match request.body_type.as_str() {
    //     "form" => {
    //         let form = request
    //             .form
    //             .iter()
    //             .map(|form| (form.field.as_str(), form.value.as_str()))
    //             .collect::<Vec<(&str, &str)>>();
    //         reqwest::Body::from(reqwest::blocking::multipart::Form::new().text("foo", "bar"))
    //     }
    //     "json" => reqwest::Body::from(request.json),
    //     _ => reqwest::Body::empty(),
    // };

    // let file_byte = std::fs::read(file_path).unwrap();
    // let part = reqwest::multipart::Part::bytes(Cow::from(file_byte)).file_name("test.txt");
    // let form = reqwest::multipart::Form::new().part("file", part);
    println!("href: {}", href);
    let mut response_result = reqwest::Client::new()
        .request(
            match request.method.as_str() {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                "DELETE" => reqwest::Method::DELETE,
                "HEAD" => reqwest::Method::HEAD,
                "OPTIONS" => reqwest::Method::OPTIONS,
                "PATCH" => reqwest::Method::PATCH,
                "TRACE" => reqwest::Method::TRACE,
                _ => reqwest::Method::GET,
            },
            href.as_str(),
        )
        .query(&params)
        .headers(headers);
    if request.body_type.as_str() == "json" {
        let json_data: Value = serde_json::from_str(request.json.as_str()).unwrap();
        response_result = response_result.json(&json_data);
    } else if request.body_type.as_str() == "form" {
        let form = request
            .form
            .iter()
            .map(|form| (form.field.as_str(), form.value.as_str()))
            .collect::<Vec<(&str, &str)>>();
        response_result = response_result.form(&form);
    }
    let response_result = response_result.send().await;
    match response_result {
        Ok(response) => {
            // 遍历 headers
            let mut headers: Vec<Value> = Vec::new();
            for (key, value) in response.headers() {
                headers.push(json!({
                    "field": key.as_str(),
                    "value": value.to_str().unwrap()
                }));
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
