//using axum
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    println!("Welcome to Elijah's API");
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/classes", get(get_classes))
        .route("/classes", post(create_class));
    
    let addr = "0.0.0.0:2000";
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to address {}: {}", addr, err);
            return;
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", err);
    }
}

async fn root() -> Json<&'static str> {
    Json("Hello, world!")
}

async fn create_class(
    Json(payload): Json<CreateClass>,
) -> (StatusCode, Json<Class>) {
    let class = Class{
        crn: 3550,
        name: payload.name,
        classroom_number: payload.classroom_number,
        subject: payload.subject,
    };

    (StatusCode::CREATED, Json(class))
}

#[derive(Deserialize)]
struct CreateClass {
    name: String,
    classroom_number: u64,
    subject: String,
}

#[derive(Serialize)]
struct Class {
    crn: u64,
    name: String,
    classroom_number: u64,
    subject: String,
}

async fn get_classes() -> Json<Vec<Class>> {
    Json(vec![
        Class {
            crn: 1234,
            name: "Math".to_string(),
            classroom_number: 101,
            subject: "Algebra".to_string(),
        },
        Class {
            crn: 5678,
            name: "Science".to_string(),
            classroom_number: 202,
            subject: "Physics".to_string(),
        },
    ])
}
