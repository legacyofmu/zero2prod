//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `toko::main`
// It also sparse you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();// No .await, no .expect
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}


