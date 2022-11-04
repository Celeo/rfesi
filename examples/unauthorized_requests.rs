use log::info;
use rfesi::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut esi = EsiBuilder::new()
        .user_agent("github.com/celeo/rfesi :: example :: unauthorized_requests")
        .build()?;
    esi.update_spec().await?;
    let alliances = esi.group_alliance().list_ids().await?;
    info!("Found {} alliances", alliances.len());

    Ok(())
}
