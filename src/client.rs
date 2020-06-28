//! Main logic

use reqwest::Client;

/// Struct to interact with the ESI.
///
/// Construct an instance of this struct using [`EsiBuilder`](./struct.EsiBuilder.html).
#[derive(Debug)]
pub struct Esi {
    pub(crate) version: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) callback_url: String,
    pub(crate) access_token: Option<String>,
    pub(crate) access_expiration: Option<u64>,
    pub(crate) refresh_token: Option<String>,

    pub(crate) client: Client,
}
