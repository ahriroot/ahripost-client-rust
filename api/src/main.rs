use axum::{
    body::Body,
    http::Request,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/data", get(get_data))
        .route("/data", post(post_data))
        .route("/data", put(put_data))
        .route("/data", delete(delete_data));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn get_data(req: Request<Body>) -> Json<Value> {
    println!("Request: {:?}", req);
    // headers
    let headers = req.headers();
    // 遍历请求头
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }
    Json(json!({ "data": 42 }))
}

// upload file
async fn post_data(req: Request<Body>) -> Json<Value> {
    println!("Request: {:?}", req);
    // headers
    let headers = req.headers();
    // 遍历请求头
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }
    Json(json!({ "data": 42 }))
}

async fn put_data(req: Request<Body>) -> Json<Value> {
    println!("Request: {:?}", req);
    // headers
    let headers = req.headers();
    // 遍历请求头
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }
    Json(json!({ "data": 42 }))
}

async fn delete_data(req: Request<Body>) -> Json<Value> {
    println!("Request: {:?}", req);
    // headers
    let headers = req.headers();
    // 遍历请求头
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }
    Json(json!({ "data": 42 }))
}
