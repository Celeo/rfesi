use crate::{Esi, EsiResult, RequestType};
use serde::Deserialize;

pub struct FactionWarfareGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct FactionLeaderboardItem {
    pub amount: u64,
    pub faction_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct FactionLeaderboardListing {
    pub active_total: Vec<FactionLeaderboardItem>,
    pub last_week: Vec<FactionLeaderboardItem>,
    pub yesterday: Vec<FactionLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
pub struct FWLeaderboards {
    pub kills: FactionLeaderboardListing,
    pub victory_points: FactionLeaderboardListing,
}

#[derive(Debug, Deserialize)]
pub struct CharacterLeaderboardItem {
    pub amount: u64,
    pub character_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct CharacterLeaderboardListing {
    pub active_total: Vec<CharacterLeaderboardItem>,
    pub last_week: Vec<CharacterLeaderboardItem>,
    pub yesterday: Vec<CharacterLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
pub struct FWCharacterLeaderboards {
    pub kills: CharacterLeaderboardListing,
    pub victory_points: CharacterLeaderboardListing,
}

#[derive(Debug, Deserialize)]
pub struct CorporationLeaderboardItem {
    pub amount: u64,
    pub corporation_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct CorporationLeaderboardListing {
    pub active_total: Vec<CorporationLeaderboardItem>,
    pub last_week: Vec<CorporationLeaderboardItem>,
    pub yesterday: Vec<CorporationLeaderboardItem>,
}

#[derive(Debug, Deserialize)]
pub struct FWCorporationLeaderboards {
    pub kills: CorporationLeaderboardListing,
    pub victory_points: CorporationLeaderboardListing,
}

#[derive(Debug, Deserialize)]
pub struct FWStatsItemRange {
    pub total: u64,
    pub last_week: u64,
    pub yesterday: u64,
}

#[derive(Debug, Deserialize)]
pub struct FWStatsItem {
    pub faction_id: u64,
    pub kills: FWStatsItemRange,
    pub pilots: u64,
    pub systems_controlled: u64,
    pub victory_points: FWStatsItemRange,
}

#[derive(Debug, Deserialize)]
pub struct FWSystem {
    pub contested: String,
    pub occupier_faction_id: u64,
    pub owner_faction_id: u64,
    pub solar_system_id: u64,
    pub victory_points: u64,
    pub victory_points_threshold: u64,
}

#[derive(Debug, Deserialize)]
pub struct FWWar {
    pub faction_id: u64,
    pub against_id: u64,
}

impl<'a> FactionWarfareGroup<'a> {
    /// Get the top 4 leaderboards of factions for total, last week, and yesterday.
    pub async fn leaderboards(&self) -> EsiResult<FWLeaderboards> {
        let path = self.esi.get_endpoint_for_op_id("get_fw_leaderboards")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get top 100 characters for total, last week, and yesterday.
    pub async fn leaderboard_characters(&self) -> EsiResult<FWCharacterLeaderboards> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_fw_leaderboards_characters")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get top 10 corporations for total, last week, and yesterday.
    pub async fn leaderboard_corporations(&self) -> EsiResult<FWCorporationLeaderboards> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_fw_leaderboards_corporations")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get FW overview stats.
    pub async fn stats(&self) -> EsiResult<Vec<FWStatsItem>> {
        let path = self.esi.get_endpoint_for_op_id("get_fw_stats")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get FW system ownership.
    pub async fn systems(&self) -> EsiResult<Vec<FWSystem>> {
        let path = self.esi.get_endpoint_for_op_id("get_fw_systems")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get FW faction information.
    pub async fn wars(&self) -> EsiResult<Vec<FWWar>> {
        let path = self.esi.get_endpoint_for_op_id("get_fw_wars")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    // more endpoints ...
}
