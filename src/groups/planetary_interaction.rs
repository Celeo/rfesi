#![allow(unused)]

use crate::Esi;

/// Endpoints for PlanetaryInteraction
pub struct PlanetaryInteractionGroup<'a> {
    pub(crate) esi: &'a Esi,
}
