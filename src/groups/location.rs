use crate::{Esi, EsiResult, RequestType};
use serde::Deserialize;

/// Endpoints for Location
pub struct LocationGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct LocationInfo {
    pub solar_system_id: u64,
    pub station_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct OnlineStatus {
    pub last_login: String,
    pub last_logout: String,
    pub logins: u64,
    pub online: bool,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CurrentShip {
    pub ship_item_id: u64,
    pub ship_name: String,
    pub ship_type_id: u64,
}

impl<'a> LocationGroup<'a> {
    api_get!(
        /// Get the character's location.
        get_location,
        "get_characters_character_id_location",
        RequestType::Authenticated,
        LocationInfo,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get if the character is currently online.
        get_online,
        "get_characters_character_id_online",
        RequestType::Authenticated,
        OnlineStatus,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get the player's current ship.
        get_ship,
        "get_characters_character_id_ship",
        RequestType::Authenticated,
        CurrentShip,
        (character_id: u64) => "{character_id}"
    );
}
