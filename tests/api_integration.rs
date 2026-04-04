use axum::{body::Body, http::Request, Router};
use tower::ServiceExt;
use serde_json::Value;
use fluxx_agent::api::StandardResponse;

#[tokio::test]
async fn test_api_standardized_response_consistency() {
    let app = Router::new().route("/test", axum::routing::get(|| async {
        StandardResponse::ok("PayloadData", "Test success message.")
    }));

    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify StandardResponse schema compliance (camelCase and existence)
    assert!(json.get("transactionUrn").is_some());
    assert_eq!(json.get("status").unwrap(), "SUCCESS");
    assert_eq!(json.get("responseMessage").unwrap(), "Test success message.");
    assert_eq!(json.get("data").unwrap(), "PayloadData");
    assert!(json.get("timestamp").is_some());
}

#[tokio::test]
async fn test_api_response_error_status() {
    let app = Router::new().route("/error", axum::routing::get(|| async {
        StandardResponse::error((), "Error message", "ERR_001", vec!["detail".to_string()])
    }));

    let response = app
        .oneshot(Request::builder().uri("/error").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.get("status").unwrap(), "FAILED");
    assert_eq!(json.get("responseKey").unwrap(), "ERR_001");
}

#[tokio::test]
async fn test_api_response_with_complex_data() {
    let app = Router::new().route("/complex", axum::routing::get(|| async {
        let data = serde_json::json!({
            "id": 123,
            "name": "test",
            "nested": {"key": "value"}
        });
        StandardResponse::ok(data, "Complex data")
    }));

    let response = app
        .oneshot(Request::builder().uri("/complex").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["data"]["id"], 123);
    assert_eq!(json["data"]["name"], "test");
}

#[tokio::test]
async fn test_api_response_transaction_urn_format() {
    let app = Router::new().route("/test", axum::routing::get(|| async {
        StandardResponse::ok("data", "message")
    }));

    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let urn = json.get("transactionUrn").unwrap().as_str().unwrap();
    assert!(urn.starts_with("urn:fluxx:tx:"));
}

#[tokio::test]
async fn test_api_response_timestamp_format() {
    let app = Router::new().route("/test", axum::routing::get(|| async {
        StandardResponse::ok("data", "message")
    }));

    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let timestamp = json.get("timestamp").unwrap().as_str().unwrap();
    assert!(!timestamp.is_empty());
}
