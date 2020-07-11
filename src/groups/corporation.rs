use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
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
#[allow(missing_docs)]
pub struct CorporationHistoryItem {
    pub alliance_id: u64,
    pub record_id: u64,
    pub start_date: String,
}

/// Endpoints for Corporation
pub struct CorporationGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> CorporationGroup<'a> {
    api_get!(
        /// Get a corporation's public info.
        get_public_info,
        "get_corporations_corporation_id",
        RequestType::Public,
        CorporationPublicInfo,
        (corporation_id: u64) => "{corporation_id}"
    );

    api_get!(
        /// Get a corporation's alliance history.
        get_history,
        "get_corporations_corporation_id_alliancehistory",
        RequestType::Public,
        Vec<CorporationHistoryItem>,
        (corporation_id: u64) => "{corporation_id}"
    );

    api_get!(
        /// Get a corporation's member list.
        ///
        /// Requires the auth'd character to be in the corporation.
        get_members,
        "get_corporations_corporation_id_members",
        RequestType::Authenticated,
        Vec<u64>,
        (corporation_id: u64) => "{corporation_id}"
    );

    api_get!(
        /// Get a list of NPC corporations.
        get_npc_corps,
        "get_corporations_npccorps",
        RequestType::Public,
        Vec<u64>,
    );

    // more endpoints ...
}
