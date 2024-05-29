use reqwest::StatusCode;

#[tokio::test]
async fn test_hello_world() {
    let resp = reqwest::get("http://localhost:8080").await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
