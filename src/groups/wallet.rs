#![allow(unused)]

use crate::Esi;

/// Endpoints for Wallet
pub struct WalletGroup<'a> {
    pub(crate) esi: &'a Esi,
}
