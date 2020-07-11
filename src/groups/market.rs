#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Market
pub struct MarketGroup<'a> {
    pub(crate) esi: &'a Esi,
}
