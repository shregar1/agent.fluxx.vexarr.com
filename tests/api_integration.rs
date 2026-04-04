use axum::{body::Body, http::Request, Router};
use tower::ServiceExt;
use serde_json::Value;
use crate::api::StandardResponse;

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
