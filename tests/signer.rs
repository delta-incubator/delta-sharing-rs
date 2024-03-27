use std::time::Duration;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::Credentials, primitives::ByteStream, Config};
use testcontainers::{clients::Cli, RunnableImage};
use testcontainers_modules::localstack::LocalStack;

use delta_sharing::signer::{s3::S3Signer, Signer};
use tokio::time::sleep;

#[tokio::test]
async fn s3_signer() {
    let docker = Cli::default();
    let image: RunnableImage<LocalStack> = LocalStack::default().into();
    let image = image
        .with_env_var(("SERVICES", "s3"))
        .with_env_var(("TEST_AWS_ACCESS_KEY_ID", "delta-sharing-key-id"))
        .with_env_var(("TEST_AWS_SECRET_ACCESS_KEY", "delta-sharing-secret-key"))
        .with_env_var(("S3_SKIP_SIGNATURE_VALIDATION", "0"));
    let container = docker.run(image);

    let host_port = container.get_host_port_ipv4(4566);
    let endpoint = format!("http://127.0.0.1:{}", host_port);

    let cfg = Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .credentials_provider(Credentials::new(
            "delta-sharing-key-id",
            "delta-sharing-secret-key",
            None,
            None,
            "test",
        ))
        .region(Region::new("us-east-1"))
        .endpoint_url(endpoint)
        .build();

    // Setup environment
    let client = aws_sdk_s3::Client::from_conf(cfg.clone());
    client
        .create_bucket()
        .bucket("bucket")
        .send()
        .await
        .unwrap();
    client
        .put_object()
        .bucket("bucket")
        .key("prefix/key.snappy.parquet")
        .body(ByteStream::from_static("hello world".as_bytes()))
        .send()
        .await
        .unwrap();

    let client = reqwest::Client::new();

    // Test S3Signer
    let signer = S3Signer::from_conf(cfg);
    let uri = "s3://bucket/prefix/key.snappy.parquet";

    let signed_url = signer
        .sign(uri, std::time::Duration::from_secs(60))
        .await
        .unwrap();
    assert!(!signed_url.is_expired());
    let response = client.get(signed_url.url()).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "hello world");

    let short_expiry_signed_url = signer
        .sign(uri, std::time::Duration::from_secs(1))
        .await
        .unwrap();
    sleep(Duration::from_secs(2)).await;
    assert!(short_expiry_signed_url.is_expired());
    let response = client
        .get(short_expiry_signed_url.url())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
}
