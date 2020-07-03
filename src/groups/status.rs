#![allow(unused)]

use crate::Esi;

/// Endpoints for Status
pub struct StatusGroup<'a> {
    pub(crate) esi: &'a Esi,
}
