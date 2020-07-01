#![allow(unused)]

use crate::{Esi, EsiError, RequestType};
use serde::Deserialize;

pub struct AssetsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
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
pub struct AssetLocationPosition {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, Deserialize)]
pub struct AssetLocation {
    pub item_id: u64,
    pub position: AssetLocationPosition,
}

#[derive(Debug, Deserialize)]
pub struct AssetName {
    pub item_id: u64,
    pub name: String,
}

impl<'a> AssetsGroup<'a> {
    /// Get a character's assets.
    pub async fn get_character_assets(&self, character_id: u64) -> Result<Vec<Asset>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id_assets")?
            .replace("{character_id}", &character_id.to_string());
        self.esi
            .query("GET", RequestType::Authenticated, &path, None, None)
            .await
    }

    /// Get locations of some of a character's assets.
    pub async fn get_character_assets_locations(
        &self,
        character_id: u64,
        item_ids: &[u64],
    ) -> Result<Vec<AssetLocation>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("post_characters_character_id_assets_locations")?
            .replace("{character_id}", &character_id.to_string());
        let body = serde_json::to_string(item_ids)?;
        self.esi
            .query("POST", RequestType::Authenticated, &path, None, Some(&body))
            .await
    }

    /// Get names of some of a character's assets.
    pub async fn get_character_assets_names(
        &self,
        character_id: u64,
        item_ids: &[u64],
    ) -> Result<Vec<AssetName>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("post_characters_character_id_assets_names")?
            .replace("{character_id}", &character_id.to_string());
        let body = serde_json::to_string(item_ids)?;
        self.esi
            .query("POST", RequestType::Authenticated, &path, None, Some(&body))
            .await
    }

    /// Get a corporation's assets.
    ///
    /// Requires the auth'd character to be a director/+ in the corp.
    pub async fn get_corporation_assets(&self, corporation_id: u64) -> Result<Vec<u64>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_corporations_corporation_id_assets")?
            .replace("{corporation_id}", &corporation_id.to_string());
        // NOTE: assuming return type; don't have the permissions to check
        self.esi
            .query("GET", RequestType::Authenticated, &path, None, None)
            .await
    }

    /// Get locations of some of a corporation's assets.
    ///
    /// Requires the auth'd character to be a director/+ in the corp.
    pub async fn get_corporation_assets_locations(
        &self,
        corporation_id: u64,
        item_ids: &[u64],
    ) -> Result<Vec<AssetLocation>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("post_corporations_corporation_id_assets_locations")?
            .replace("{corporation_id}", &corporation_id.to_string());
        let body = serde_json::to_string(item_ids)?;
        // NOTE: assuming return type; don't have the permissions to check
        self.esi
            .query("POST", RequestType::Authenticated, &path, None, Some(&body))
            .await
    }

    /// Get names of some of a corporation's assets.
    ///
    /// Requires the auth'd character to be a director/+ in the corp.
    pub async fn get_corporation_assets_names(
        &self,
        corporation_id: u64,
        item_ids: &[u64],
    ) -> Result<Vec<AssetName>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("post_corporations_corporation_id_assets_names")?
            .replace("{corporation_id}", &corporation_id.to_string());
        let body = serde_json::to_string(item_ids)?;
        // NOTE: assuming return type; don't have the permissions to check
        self.esi
            .query("POST", RequestType::Authenticated, &path, None, Some(&body))
            .await
    }
}
