mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use server::{
    routes::create_router,
    state::AppState,
    services::notify::NoopNotifier,
};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_auth_flow() {
    // Setup
    let db = common::setup_test_db().await;
    let state = AppState {
        db: db.clone(),
        notifier: Arc::new(NoopNotifier),
    };
    let app = create_router(state);

    // 1. Register
    let username = format!("user_{}", uuid::Uuid::new_v4());
    let payload = serde_json::json!({
        "username": username,
        "password": "password123"
    });
    
    let req = Request::builder()
        .method("POST")
        .uri("/register")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    
    let token = body["token"].as_str().expect("Token not found in register response");
    // Ensure we can parse user_id
    let _user_id = body["id"].as_str().expect("User ID not found");

    // 2. Login
    let payload = serde_json::json!({
        "username": username,
        "password": "password123"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    let login_token = body["token"].as_str().expect("Token not found in login response");
    
    // 3. Access Protected Route (List Accounts) - Success
    let req = Request::builder()
        .method("GET")
        .uri("/accounts")
        .header("Authorization", format!("Bearer {}", login_token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 4. Access Protected Route - No Token
    let req = Request::builder()
        .method("GET")
        .uri("/accounts")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 5. Access Protected Route - Invalid Token
    let req = Request::builder()
        .method("GET")
        .uri("/accounts")
        .header("Authorization", "Bearer invalid.token.here")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
