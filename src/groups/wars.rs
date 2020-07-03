#![allow(unused)]

use crate::Esi;

/// Endpoints for Wars
pub struct WarsGroup<'a> {
    pub(crate) esi: &'a Esi,
}
