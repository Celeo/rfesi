#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Universe
pub struct UniverseGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Constellation {
    pub constellation_id: i32,
    pub name: String,
    pub position: Position,
    pub region_id: i32,
    pub systems: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Region {
    pub constellations: Vec<i32>,
    pub description: Option<String>,
    pub name: String,
    pub region_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct SystemPlanet {
    pub asteroid_belts: Option<Vec<i32>>,
    pub moons: Option<Vec<i32>>,
    pub planet_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct System {
    pub constellation_id: i32,
    pub name: String,
    pub planets: Option<Vec<SystemPlanet>>,
    pub position: Position,
    pub security_class: Option<String>,
    pub security_status: f64,
    pub star_id: Option<i32>,
    pub stargates: Option<Vec<i32>>,
    pub stations: Option<Vec<i32>>,
    pub system_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct TypeDogmaAttribute {
    pub attribute_id: i32,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct TypeDogmaEffect {
    pub effect_id: i32,
    pub is_default: bool,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Type {
    pub capacity: Option<f64>,
    pub description: String,
    pub dogma_attributes: Option<Vec<TypeDogmaAttribute>>,
    pub dogma_effects: Option<Vec<TypeDogmaEffect>>,
    pub graphic_id: Option<i32>,
    pub group_id: i32,
    pub icon_id: Option<i32>,
    pub market_group_id: Option<i32>,
    pub mass: Option<f64>,
    pub name: String,
    pub packaged_volume: Option<f64>,
    pub portion_size: Option<i32>,
    pub published: bool,
    pub radius: Option<f64>,
    pub type_id: i32,
    pub volume: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Station {
    pub max_dockable_ship_volume: f64,
    pub name: String,
    pub office_rental_cost: f64,
    pub owner: Option<i32>,
    pub position: Position,
    pub race_id: Option<i32>,
    pub reprocessing_efficiency: f64,
    pub reprocessing_stations_take: f64,
    pub services: Vec<String>,
    pub station_id: i32,
    pub system_id: i32,
    pub type_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Structure {
    pub name: String,
    pub owner_id: i32,
    pub position: Position,
    pub solar_system_id: i32,
    pub type_id: Option<i32>,
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

    api_get!(
        /// Information about a station
        get_station,
        "get_universe_stations_station_id",
        RequestType::Public,
        Station,
        (station_id: u64) => "{station_id}"
    );

    api_get!(
        /// Returns information on requested structure if you are on the ACL. Otherwise, returns “Forbidden” for all inputs.
        get_structure,
        "get_universe_structures_structure_id",
        RequestType::Authenticated,
        Structure,
        (structure_id: u64) => "{structure_id}"
    );
}
