#![allow(unused)]

use crate::Esi;

/// Endpoints for Loyalty
pub struct LoyaltyGroup<'a> {
    pub(crate) esi: &'a Esi,
}
