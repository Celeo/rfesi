#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Wallet
pub struct WalletGroup<'a> {
    pub(crate) esi: &'a Esi,
}
