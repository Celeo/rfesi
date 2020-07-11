#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Dogma
pub struct DogmaGroup<'a> {
    pub(crate) esi: &'a Esi,
}
