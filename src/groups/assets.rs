use crate::prelude::*;
use serde::Deserialize;

/// Endpoints for Assets
pub struct AssetsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Asset {
    pub is_singleton: bool,
    pub item_id: u64,
    pub location_flag: String,
    pub location_id: u64,
    pub location_type: String,
    pub quantity: u64,
    pub type_id: u64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AssetLocationPosition {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AssetLocation {
    pub item_id: u64,
    pub position: AssetLocationPosition,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AssetName {
    pub item_id: u64,
    pub name: String,
}

impl<'a> AssetsGroup<'a> {
    api_get!(
        /// Get a character's assets.
        get_character_assets,
        "get_characters_character_id_assets",
        RequestType::Authenticated,
        Vec<Asset>,
        (character_id: u64) => "{character_id}"
    );

    api_post!(
        /// Get locations of some of a character's assets.
        get_character_assets_locations,
        "post_characters_character_id_assets_locations",
        RequestType::Authenticated,
        Vec<AssetLocation>,
        (character_id: u64) => "{character_id}",
        item_ids: &[u64],
    );

    api_post!(
        /// Get names of some of a character's assets.
        get_character_assets_names,
        "post_characters_character_id_assets_names",
        RequestType::Authenticated,
        Vec<AssetName>,
        (character_id: u64) => "{character_id}",
        item_ids: &[u64],
    );

    // NOTE: assuming return type; don't have the permissions to check
    api_get!(
        /// Get a corporation's assets.
        ///
        /// Requires the auth'd character to be a director/+ in the corp.
        get_corporation_assets,
        "get_corporations_corporation_id_assets",
        RequestType::Authenticated,
        Vec<u64>,
        (corporation_id: u64) => "{corporation_id}"
    );

    api_post!(
        /// Get locations of some of a corporation's assets.
        ///
        /// Requires the auth'd character to be a director/+ in the corp.
        get_corporation_assets_locations,
        "post_corporations_corporation_id_assets_locations",
        RequestType::Authenticated,
        Vec<AssetLocation>,
        (corporation_id: u64) => "{corporation_id}",
        item_ids: &[u64],
    );

    api_post!(
        /// Get names of some of a corporation's assets.
        ///
        /// Requires the auth'd character to be a director/+ in the corp.
        get_corporation_assets_names,
        "post_corporations_corporation_id_assets_names",
        RequestType::Authenticated,
        Vec<AssetName>,
        (corporation_id: u64) => "{corporation_id}",
        item_ids: &[u64],
    );
}
