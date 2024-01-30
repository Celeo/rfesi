use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct CorporationPublicInfo {
    pub alliance_id: Option<i32>,
    pub ceo_id: i32,
    pub creator_id: i32,
    pub date_founded: Option<String>,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub home_station_id: Option<i32>,
    pub member_count: i32,
    pub name: String,
    pub shares: Option<u64>,
    pub tax_rate: f64,
    pub ticker: Option<String>,
    pub url: Option<String>,
    pub war_eligible: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CorporationHistoryItem {
    pub alliance_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub record_id: i32,
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
        (corporation_id: i32) => "{corporation_id}"
    );

    api_get!(
        /// Get a corporation's alliance history.
        get_history,
        "get_corporations_corporation_id_alliancehistory",
        RequestType::Public,
        Vec<CorporationHistoryItem>,
        (corporation_id: i32) => "{corporation_id}"
    );

    api_get!(
        /// Get a corporation's member list.
        ///
        /// Requires the auth'd character to be in the corporation.
        get_members,
        "get_corporations_corporation_id_members",
        RequestType::Authenticated,
        Vec<u64>,
        (corporation_id: i32) => "{corporation_id}"
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
