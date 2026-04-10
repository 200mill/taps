use std::sync::Arc;
use zako3_tap_sdk::tap;

pub mod papago;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    tracing_subscriber::fmt::init();

    let tap_id = std::env::var("PAPAGO_TAP_ID").expect("PAPAGO_TAP_ID env var is required");
    let api_token =
        std::env::var("PAPAGO_API_TOKEN").expect("PAPAGO_API_TOKEN env var is required");

    tap()
        //.cert_pem("cert.pem")
        .hub("api.zako.ac")
        .tap_id(&tap_id)
        .friendly_name("Papago TTS Tap")
        .api_token(&api_token)
        .selection_weight(1.0)
        .run(Arc::new(papago::PapagoTapHandler))
        .await?;

    Ok(())
}
