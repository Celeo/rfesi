use crate::prelude::*;

/// Endpoints for FactionWarfare
pub struct FactionWarfareGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FactionLeaderboardItem {
    pub amount: Option<i32>,
    pub faction_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FactionLeaderboardListing {
    pub active_total: Vec<FactionLeaderboardItem>,
    pub last_week: Vec<FactionLeaderboardItem>,
    pub yesterday: Vec<FactionLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWLeaderboards {
    pub kills: FactionLeaderboardListing,
    pub victory_points: FactionLeaderboardListing,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterLeaderboardItem {
    pub amount: i32,
    pub character_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterLeaderboardListing {
    pub active_total: Vec<CharacterLeaderboardItem>,
    pub last_week: Vec<CharacterLeaderboardItem>,
    pub yesterday: Vec<CharacterLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWCharacterLeaderboards {
    pub kills: CharacterLeaderboardListing,
    pub victory_points: CharacterLeaderboardListing,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CorporationLeaderboardItem {
    pub amount: i32,
    pub corporation_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CorporationLeaderboardListing {
    pub active_total: Vec<CorporationLeaderboardItem>,
    pub last_week: Vec<CorporationLeaderboardItem>,
    pub yesterday: Vec<CorporationLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWCorporationLeaderboards {
    pub kills: CorporationLeaderboardListing,
    pub victory_points: CorporationLeaderboardListing,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWStatsItemRange {
    pub total: i32,
    pub last_week: i32,
    pub yesterday: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWStatsItem {
    pub faction_id: u32,
    pub kills: FWStatsItemRange,
    pub pilots: u32,
    pub systems_controlled: u32,
    pub victory_points: FWStatsItemRange,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWSystem {
    pub contested: String,
    pub occupier_faction_id: u8,
    pub owner_faction_id: u32,
    pub solar_system_id: u32,
    pub victory_points: u32,
    pub victory_points_threshold: u32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FWWar {
    pub faction_id: i32,
    pub against_id: i32,
}

impl<'a> FactionWarfareGroup<'a> {
    api_get!(
        /// Get the top 4 leaderboards of factions for total, last week, and yesterday.
        leaderboards,
        "get_fw_leaderboards",
        RequestType::Public,
        FWLeaderboards,
    );

    api_get!(
        /// Get top 100 characters for total, last week, and yesterday.
        leaderboard_characters,
        "get_fw_leaderboards_characters",
        RequestType::Public,
        FWCharacterLeaderboards,
    );

    api_get!(
        /// Get top 10 corporations for total, last week, and yesterday.
        leaderboard_corporations,
        "get_fw_leaderboards_corporations",
        RequestType::Public,
        FWCorporationLeaderboards,
    );

    api_get!(
        /// Get FW overview stats.
        stats,
        "get_fw_stats",
        RequestType::Public,
        Vec<FWStatsItem>,
    );

    api_get!(
        /// Get FW system ownership.
        systems,
        "get_fw_systems",
        RequestType::Public,
        Vec<FWSystem>,
    );

    api_get!(
        /// Get FW faction information.
        wars,
        "get_fw_wars",
        RequestType::Public,
        Vec<FWWar>,
    );

    // more endpoints ...
}
