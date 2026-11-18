mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use pulse_server::router;
use rstest::rstest;
use tower::ServiceExt;

#[rstest]
#[case("/healthz", StatusCode::OK, "ok")]
#[case("/missing", StatusCode::NOT_FOUND, "")]
#[tokio::test]
async fn routes_respond(#[case] path: &str, #[case] status: StatusCode, #[case] body: &str) {
    let response = router(common::lazy_state())
        .oneshot(Request::get(path).body(Body::empty()).expect("request"))
        .await
        .expect("response");
    assert_eq!(response.status(), status);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    assert_eq!(bytes.as_ref(), body.as_bytes());
}
