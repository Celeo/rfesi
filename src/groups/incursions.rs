use crate::{Esi, EsiResult, RequestType};
use serde::Deserialize;

pub struct IncursionsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct Incursion {
    pub constellation_id: u64,
    pub faction_id: u64,
    pub has_boss: bool,
    pub infested_solar_systems: Vec<u64>,
    pub influence: f64,
    pub staging_solar_system_id: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub incursion_type: String,
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
