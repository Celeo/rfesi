use rfesi::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let _esi = EsiBuilder::new()
        .user_agent("github.com/celeo/rfesi :: example :: auth_from_refresh_token")
        .client_id("abc")
        .client_secret("def")
        .callback_url("http://localhost:5000/esi/callback")
        .scope("g h i")
        .build()?;

    // esi.use_refresh_token("jkl").await?;

    Ok(())
}
