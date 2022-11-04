use log::info;
use rfesi::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let esi = EsiBuilder::new()
        .user_agent("github.com/celeo/rfesi :: example :: auth_fresh")
        .client_id("abc")
        .client_secret("def")
        .callback_url("http://localhost:5000/esi/callback")
        .scope("g h i")
        .build()?;
    info!("Send your users to {}", esi.get_authorize_url()?.0);

    Ok(())
}
