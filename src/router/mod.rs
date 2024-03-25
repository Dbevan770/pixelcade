use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use crate::auth::register;

pub async fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/register", post(register_handler))
}

async fn login_handler() -> Result<String, StatusCode> {
    println!("Login handler called");

    Ok(StatusCode::OK.to_string())
}

async fn register_handler() -> Result<String, StatusCode> {
    println!("Register handler called");

    register::register().await;

    Ok(StatusCode::OK.to_string())
}

async fn handler() -> &'static str {
    "Hello World!"
}
