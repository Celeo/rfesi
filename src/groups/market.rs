#![allow(unused)]

use crate::Esi;

/// Endpoints for Market
pub struct MarketGroup<'a> {
    pub(crate) esi: &'a Esi,
}
