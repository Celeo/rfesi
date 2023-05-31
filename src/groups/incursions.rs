use crate::prelude::*;

/// Endpoints for Incursions
pub struct IncursionsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Incursion {
    pub constellation_id: u64,
    pub faction_id: u64,
    pub has_boss: bool,
    pub infested_solar_systems: Vec<u32>,
    pub influence: f64,
    pub staging_solar_system_id: u64,
    pub state: String,
    #[serde(rename = "type")]
    pub incursion_type: String,
}

impl<'a> IncursionsGroup<'a> {
    api_get!(
        /// Get the current incursions.
        list,
        "get_incursions",
        RequestType::Public,
        Vec<Incursion>,
    );
}
