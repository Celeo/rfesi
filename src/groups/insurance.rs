#![allow(unused)]

use crate::Esi;

/// Endpoints for Insurance
pub struct InsuranceGroup<'a> {
    pub(crate) esi: &'a Esi,
}
