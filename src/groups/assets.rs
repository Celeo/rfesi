use crate::prelude::*;

/// Endpoints for Assets
pub struct AssetsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Asset {
    pub is_blueprint_copy: Option<bool>,
    pub is_singleton: bool,
    pub item_id: i64,
    pub location_flag: String,
    pub location_id: i64,
    pub location_type: String,
    pub quantity: i32,
    pub type_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AssetLocationPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct AssetLocation {
    pub item_id: i64,
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
        (character_id: i32) => "{character_id}"
    );

    api_post!(
        /// Get locations of some of a character's assets.
        get_character_assets_locations,
        "post_characters_character_id_assets_locations",
        RequestType::Authenticated,
        Vec<AssetLocation>,
        (character_id: i32) => "{character_id}",
        item_ids: &[i64],
    );

    api_post!(
        /// Get names of some of a character's assets.
        get_character_assets_names,
        "post_characters_character_id_assets_names",
        RequestType::Authenticated,
        Vec<AssetName>,
        (character_id: i32) => "{character_id}",
        item_ids: &[u64],
    );

    api_get!(
        /// Get a corporation's assets.
        ///
        /// Requires the auth'd character to be a director/+ in the corp.
        get_corporation_assets,
        "get_corporations_corporation_id_assets",
        RequestType::Authenticated,
        Vec<Asset>,
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
