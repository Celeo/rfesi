use crate::prelude::*;

/// Endpoints for Alliance
pub struct AllianceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct AllianceInfo {
    pub creator_corporation_id: i32,
    pub creator_id: i32,
    pub date_founded: String,
    pub executor_corporation_id: Option<i32>,
    pub faction_id: Option<i32>,
    pub name: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AllianceIcons {
    pub px128x128: Option<String>,
    pub px64x64: Option<String>,
}

impl<'a> AllianceGroup<'a> {
    api_get!(
        /// Get a list of alliance IDs.
        list_ids,
        "get_alliances",
        RequestType::Public,
        Vec<i32>,
    );

    api_get!(
        /// Get public information about an alliance.
        get_info,
        "get_alliances_alliance_id",
        RequestType::Public,
        AllianceInfo,
        (alliance_id: i32) => "{alliance_id}"
    );

    api_get!(
        /// Get list of corporation IDs in an alliance.
        get_alliance_corporations,
        "get_alliances_alliance_id_corporations",
        RequestType::Public,
        Vec<i32>,
        (alliance_id: i32) => "{alliance_id}"
    );

    api_get!(
        /// Get paths to the alliance's icons on the image server.
        get_alliance_icons,
        "get_alliances_alliance_id_icons",
        RequestType::Public,
        AllianceIcons,
        (alliance_id: i32) => "{alliance_id}"
    );
}
