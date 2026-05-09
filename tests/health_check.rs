#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to send");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0))
}

fn spawn_app() {
    let server = rust_actix::run().expect("Failed to start server");

    let _ = tokio::spawn(server);
}
