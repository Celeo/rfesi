use crate::prelude::*;

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
    // NOTE unknown type; I haven't played in a long time
    api_get!(
        /// Get a character's recent kills & losses.
        get_character_recent,
        "get_characters_character_id_killmails_recent",
        RequestType::Authenticated,
        Vec<serde_json::Value>,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get a killmail.
        get_killmail,
        "get_killmails_killmail_id_killmail_hash",
        RequestType::Public,
        Killmail,
        (killmail_id: u64) => "{killmail_id}",
        (killmail_hash: &str) => "{killmail_hash}"
    );

    // more endpoints ...
}
