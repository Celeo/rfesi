use crate::prelude::*;

/// Endpoints for Killmails
pub struct KillmailsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct RecentKillMail {
    pub killmail_hash: String,
    pub killmail_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailAttacker {
    pub alliance_id: Option<i32>,
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub damage_done: i32,
    pub final_blow: bool,
    pub security_status: f64,
    pub ship_type_id: Option<i32>,
    pub weapon_type_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailItem {
    pub flag: i32,
    pub item_type_id: i32,
    pub quantity_destroyed: Option<i64>,
    pub quantity_dropped: Option<i64>,
    pub singleton: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct KillmailVictim {
    pub alliance_id: Option<i32>,
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub damage_taken: i32,
    pub faction_id: Option<i32>,
    pub items: Option<Vec<KillmailItem>>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Killmail {
    pub killmail_id: i32,
    pub killmail_type: String,
    pub solar_system_id: i32,
    pub moon_id: Option<i32>,
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
        Vec<RecentKillMail>,
        (character_id: i32) => "{character_id}"
    );

    api_get!(
        /// Get a killmail.
        get_killmail,
        "get_killmails_killmail_id_killmail_hash",
        RequestType::Public,
        Killmail,
        (killmail_id: i32) => "{killmail_id}",
        (killmail_hash: &str) => "{killmail_hash}"
    );

    // more endpoints ...
}
