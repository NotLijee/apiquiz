//using axum
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/classes", get(get_classes))
        .route("/classes", post(create_class));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
async fn root() -> &'static str {
    "Hello, world!"
}
async fn create_class(
    Json(payload): Json<CreateClass>,
) -> (StatusCode, Json<Class>) {
    let class = Class{
    crn: 3550,
    name: payload.name,
    };

    (StatusCode::CREATED, Json(class))
}
#[derive(Deserialize)]
struct CreateClass {
    name: String,
}

#[derive(Serialize)]
struct Class {
    crn: u64,
    name: String 
}


async fn get_classes() -> &'static str {
    "the class list"
 }
