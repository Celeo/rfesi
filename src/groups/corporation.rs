#![allow(unused)]

use crate::{Esi, EsiError, RequestType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CorporationPublicInfo {
    pub alliance_id: Option<u64>,
    pub ceo_id: u64,
    pub creator_id: u64,
    pub date_founded: String,
    pub description: String,
    pub home_station_id: u64,
    pub member_count: u64,
    pub name: String,
    pub shares: u64,
    pub tax_rate: f32,
    pub ticket: String,
    pub url: String,
    pub war_eligible: bool,
}

#[derive(Debug, Deserialize)]
pub struct CorporationHistoryItem {
    pub alliance_id: u64,
    pub record_id: u64,
    pub start_date: String,
}

pub struct CorporationGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> CorporationGroup<'a> {
    /// Get a corporation's public info.
    pub async fn get_public_info(
        &self,
        corporation_id: u64,
    ) -> Result<CorporationPublicInfo, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_corporations_corporation_id")?
            .replace("{corporation_id}", &corporation_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get a corporation's alliance history.
    pub async fn get_history(
        &self,
        corporation_id: u64,
    ) -> Result<Vec<CorporationHistoryItem>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_corporations_corporation_id_alliancehistory")?
            .replace("{corporation_id}", &corporation_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get a corporation's member list.
    ///
    /// Requires the auth'd character to be in the corporation.
    pub async fn get_members(&self, corporation_id: u64) -> Result<Vec<u64>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_corporations_corporation_id_members")?
            .replace("{corporation_id}", &corporation_id.to_string());
        self.esi
            .query("GET", RequestType::Authenticated, &path, None, None)
            .await
    }

    /// Get a list of NPC corporations.
    pub async fn get_npc_corps(&self) -> Result<Vec<u64>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_corporations_npccorps")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    // more endpoints ...
}
