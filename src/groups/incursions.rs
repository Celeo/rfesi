use crate::{Esi, EsiResult, RequestType};
use serde::Deserialize;

pub struct IncursionsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct Incursion {
    constellation_id: u64,
    faction_id: u64,
    has_boss: bool,
    infested_solar_systems: Vec<u64>,
    influence: f64,
    staging_solar_system_id: i64,
    state: String,
    #[serde(rename = "type")]
    incursion_type: String,
}

impl<'a> IncursionsGroup<'a> {
    /// Get the current incursions.
    pub async fn list(&self) -> EsiResult<Vec<Incursion>> {
        let path = self.esi.get_endpoint_for_op_id("get_incursions")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }
}
