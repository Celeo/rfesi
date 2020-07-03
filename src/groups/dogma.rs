#![allow(unused)]

use crate::Esi;

/// Endpoints for Dogma
pub struct DogmaGroup<'a> {
    pub(crate) esi: &'a Esi,
}
