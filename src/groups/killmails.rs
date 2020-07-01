use crate::{Esi, EsiError, RequestType};
use serde::Deserialize;

pub struct KillmailsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
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
pub struct KillmailItem {
    pub flag: u64,
    pub item_type_id: u64,
    pub quantity_dropped: u64,
    pub singleton: u8,
}

#[derive(Debug, Deserialize)]
pub struct KillmailVictim {
    pub character_id: u64,
    pub corporation_id: u64,
    pub damage_taken: u64,
    pub faction_id: u64,
    pub items: Vec<KillmailItem>,
}

#[derive(Debug, Deserialize)]
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
    ) -> Result<Vec<serde_json::Value>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id_killmails_recent")?
            .replace("{character_id}", &character_id.to_string());
        // FIXME unknown type; I haven't played in a long time
        self.esi
            .query("GET", RequestType::Authenticated, &path, None, None)
            .await
    }

    /// Get a killmail.
    pub async fn get_killmail(
        &self,
        killmail_id: u64,
        killmail_hash: &str,
    ) -> Result<Killmail, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_killmails_killmail_id_killmail_hash")?
            .replace("{killmail_id}", &killmail_id.to_string())
            .replace("{killmail_hash}", killmail_hash);
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }
}
