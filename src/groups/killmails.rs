use crate::{http_get, Esi, EsiResult, RequestType};
use serde::Deserialize;

/// Endpoints for Killmails
pub struct KillmailsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailAttacker {
    pub alliance_id: u64,
    pub character_id: u64,
    pub corporation_id: u64,
    pub damage_done: u64,
    pub final_blow: bool,
    pub security_status: f64,
    pub ship_type_id: u64,
    pub weapon_type_id: u64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailItem {
    pub flag: u64,
    pub item_type_id: u64,
    pub quantity_dropped: u64,
    pub singleton: u8,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailVictim {
    pub character_id: u64,
    pub corporation_id: u64,
    pub damage_taken: u64,
    pub faction_id: u64,
    pub items: Vec<KillmailItem>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Killmail {
    pub killmail_id: u64,
    pub killmail_type: String,
    pub solar_system_id: u64,
    pub attackers: Vec<KillmailAttacker>,
    pub victim: KillmailVictim,
}

impl<'a> KillmailsGroup<'a> {
        /// Get a character's recent kills & losses.
    pub async fn get_character_recent(
        &self,
        character_id: u64,
    ) -> EsiResult<Vec<serde_json::Value>> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id_killmails_recent")?
            .replace("{character_id}", &character_id.to_string());
        // FIXME unknown type; I haven't played in a long time
        self.esi
            .query("GET", RequestType::Authenticated, &path, None, None)
            .await
    }

    http_get!(
        /// Get a killmail.
        get_killmail,
        "get_killmails_killmail_id_killmail_hash",
        Killmail,
        (killmail_id: u64) => "{killmail_id}",
        (killmail_hash: &str) => "{killmail_hash}"
    );

    // more endpoints ...
}
