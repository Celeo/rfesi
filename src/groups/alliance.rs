use crate::prelude::*;
use serde::Deserialize;

/// Endpoints for Alliance
pub struct AllianceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AllianceInfo {
    pub creator_corporation_id: u64,
    pub creator_id: u64,
    pub date_founded: String,
    pub executor_corporation_id: u64,
    pub name: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AllianceIcons {
    pub px128x128: String,
    pub px64x64: String,
}

impl<'a> AllianceGroup<'a> {
    api_get!(
        /// Get a list of alliance IDs.
        list_ids,
        "get_alliances",
        RequestType::Public,
        Vec<u64>,
    );

    api_get!(
        /// Get public information about an alliance.
        get_info,
        "get_alliances_alliance_id",
        RequestType::Public,
        AllianceInfo,
        (alliance_id: u64) => "{alliance_id}"
    );

    api_get!(
        /// Get list of corporation IDs in an alliance.
        get_alliance_corporations,
        "get_alliances_alliance_id_corporations",
        RequestType::Public,
        Vec<u64>,
        (alliance_id: u64) => "{alliance_id}"
    );

    api_get!(
        /// Get paths to the alliance's icons on the image server.
        get_alliance_icons,
        "get_alliances_alliance_id_icons",
        RequestType::Public,
        AllianceIcons,
        (alliance_id: u64) => "{alliance_id}"
    );
}
