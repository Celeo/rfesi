use crate::{Esi, EsiError, UrlBase};
use serde::Deserialize;

pub struct AllianceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct AllianceInfo {
    creator_corporation_id: u64,
    creator_id: u64,
    date_founded: String,
    executor_corporation_id: u64,
    name: String,
    ticker: String,
}

impl<'a> AllianceGroup<'a> {
    /// Get a list of alliance IDs.
    pub async fn list_ids(&self) -> Result<Vec<u64>, EsiError> {
        let path = self.esi.get_endpoint_for_op_id("get_alliances")?;
        self.esi.query("GET", UrlBase::Public, &path, None).await
    }

    /// Get public information about an alliance by ID.
    pub async fn get_info(&self, id: u64) -> Result<AllianceInfo, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_alliances_alliance_id")?;
        let path = path.replace("{alliance_id}", &id.to_string());
        self.esi.query("GET", UrlBase::Public, &path, None).await
    }
}
