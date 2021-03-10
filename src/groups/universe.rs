#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Universe
pub struct UniverseGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Constellation {
    constellation_id: i32,
    name: String,
    position: Position,
    region_id: i32,
    systems: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Region {
    constellations: Vec<i32>,
    description: Option<String>,
    name: String,
    region_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct SystemPlanet {
    asteroid_belts: Option<Vec<i32>>,
    moons: Option<Vec<i32>>,
    planet_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct System {
    constellation_id: i32,
    name: String,
    planets: Vec<SystemPlanet>,
    position: Position,
    security_class: Option<String>,
    security_status: f64,
    star_id: Option<i32>,
    stargates: Option<Vec<i32>>,
    stations: Option<Vec<i32>>,
    system_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct TypeDogmaAttribute {
    attribute_id: i32,
    value: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct TypeDogmaEffect {
    effect_id: i32,
    is_default: bool,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Type {
    capacity: Option<f64>,
    description: String,
    dogma_attributes: Option<Vec<TypeDogmaAttribute>>,
    dogma_effects: Option<Vec<TypeDogmaEffect>>,
    graphic_id: Option<i32>,
    group_id: i32,
    icon_id: Option<i32>,
    market_group_id: Option<i32>,
    mass: Option<f64>,
    name: String,
    packaged_volume: Option<f64>,
    portion_size: Option<i32>,
    published: bool,
    radius: Option<f64>,
    type_id: i32,
    volume: Option<f64>,
}

impl<'a> UniverseGroup<'a> {
    api_get!(
        /// Get a list of constellation ids
        get_constellation_ids,
        "get_universe_constellations",
        RequestType::Public,
        Vec<i32>,
    );

    api_get!(
        /// Get information on a constellation
        get_constellation,
        "get_universe_constellations_constellation_id",
        RequestType::Public,
        Constellation,
        (constellation_id: i32) => "{constellation_id}"
    );

    api_get!(
        /// Get a list of region ids
        get_region_ids,
        "get_universe_regions",
        RequestType::Public,
        Vec<i32>,
    );

    api_get!(
        /// Get information on a region
        get_region,
        "get_universe_regions_region_id",
        RequestType::Public,
        Region,
        (region_id: i32) => "{region_id}"
    );

    api_get!(
        /// Get a list of system ids
        get_system_ids,
        "get_universe_systems",
        RequestType::Public,
        Vec<i32>,
    );

    api_get!(
        /// Get information on a system
        get_system,
        "get_universe_systems_system_id",
        RequestType::Public,
        System,
        (system_id: i32) => "{system_id}"
    );

    api_get!(
        /// Get a list of type ids
        get_type_ids,
        "get_universe_types",
        RequestType::Public,
        Vec<i32>,
    );

    api_get!(
        /// Get information on a type
        get_type,
        "get_universe_types_type_id",
        RequestType::Public,
        Type,
        (type_id: i32) => "{type_id}"
    );
}
