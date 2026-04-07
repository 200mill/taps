use std::sync::Arc;
use zako3_tap_sdk::tap;

pub mod gtts;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    tracing_subscriber::fmt::init();

    let tap_id = std::env::var("GOOGLE_TAP_ID").unwrap();
    let api_token = std::env::var("GOOGLE_API_TOKEN").unwrap();

    tap()
        .cert_pem("cert.pem")
        .hub("127.0.0.1:4001")
        .tap_id(&tap_id)
        .friendly_name("Google TTS Tap")
        .api_token(&api_token)
        .selection_weight(1.0)
        .run(Arc::new(gtts::GttsTapHandler))
        .await?;

    Ok(())
}
