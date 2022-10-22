use axum::error_handling::HandleErrorExt;
use axum::http::{StatusCode, Method};
use axum::routing::{
    service_method_routing, get, post
};
use axum::{
    response::IntoResponse,
    Json
};
use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    // trace::{DefaultMakeSpan, TraceLayer}
};
use serde::{Deserialize};
// use std::thread;

#[allow(unused)]
async fn axum_rs_txt() -> String {
    std::fs::read_to_string("static/axum-rs.txt").unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[allow(unused)]
async fn login(Json(input): Json<Login>) -> impl IntoResponse {
    Json(serde_json::json!({
        "token": "hello token",
        "username": input.username,
        "password": input.password,
    }))
}

async fn userinfo() -> impl IntoResponse {
    Json(serde_json::json!({
            "username": "admin",
            "userid":   "001",
            "email":    "admin@example.com",
            "data": {
                "username": "admin",
                "userid":   "001",
                "email":    "admin@example.com",
                "avatar":   "https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif",
                "name":     "Super Admin",
            }
    }))
}

extern crate chrono;

use chrono::Local;

async fn tablelist() -> impl IntoResponse {
    // let mut maps = Vec::new();
    let status = vec!["published", "draft", "deleted"];
    let date = Local::now();

    // for i in 0..30 {
    //     let current = i % 3;
    //     maps.push({
    //         "id": "id",
    //         "title": format!("title-{}", i),
    //         "status": status.get(current),
    //         "author": Uuid::new_v4(),
    //         "display_time": format!("{}", date.format("%Y-%m-%d][%H:%M:%S")),
    //         "pageviews": format!("pageviews-{}", i),
    //     })
    // }

    Json(serde_json::json!({
        "data": {
            "total": 30,
            "items": [
                {
                    "id": "id",
                    "title": format!("title-{}", 1),
                    "status": status.get(1),
                    "author": format!("author-{}", 2),
                    "display_time": format!("{}", date.format("%Y-%m-%d][%H:%M:%S")),
                    "pageviews": format!("pageviews-{}", 3),
                }
            ]
        }
    }))
}

pub async fn initialize() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_websockets=debug,tower_http=debug")
    }
    // tracing_subscriber::fmt::init();
    tracing::debug!("server web on {} stritng", "127.0.0.1:8000");
  
    //let app = Router::new().route("/static/axum-rs.txt", axum::routing::get(axum_rs_txt));
    let app = Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .route("/admin/auth/login",post(login))
        .route("/admin/auth/logout",post(|| async {"Success"}))
        .route("/admin/userinfo",get(userinfo))
        .route("/admin/tablelist",get(tablelist))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::GET])
        )
        .nest(
            "/dashboard",
            service_method_routing::get(ServeDir::new("dist"))
                .handle_error(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("处理静态资源出错：{:?}", err),
                    )
                })
        ).layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::GET])
        );
  
    tracing::debug!("listening on {}", "127.0.0.1:8000");
    // thread::spawn(move || async {
    //   axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
    //       .serve(app.into_make_service())
    //       .await
    //       .unwrap();
    // });

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}