#![allow(unused)]

use crate::Esi;

/// Endpoints for Fleets
pub struct FleetsGroup<'a> {
    pub(crate) esi: &'a Esi,
}
