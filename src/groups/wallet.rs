#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Wallet
pub struct WalletGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> WalletGroup<'a> {

    api_get!(
        /// Returns a character’s wallet balance
        get_wallet,
        "get_characters_character_id_wallet",
        RequestType::Authenticated,
        f64,
        (character_id: i32) => "{character_id}"
    );

}