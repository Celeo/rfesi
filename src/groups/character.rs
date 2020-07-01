#![allow(unused)]

use crate::{Esi, EsiError, RequestType};
use serde::Deserialize;

pub struct CharacterGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct CharacterPublicInfo {
    pub alliance_id: u64,
    pub ancestry_id: u16,
    pub birthday: String,
    pub corporation_id: u64,
    pub description: String,
    pub gender: String,
    pub name: String,
    pub race_id: u16,
    pub security_status: f64,
}

#[derive(Debug, Deserialize)]
pub struct CorporationHistoryItem {
    pub corporation_id: u64,
    pub record_id: u64,
    pub start_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CharacterPortraitInfo {
    pub px128x128: String,
    pub px256x256: String,
    pub px512x512: String,
    pub px64x64: String,
}

#[derive(Debug, Deserialize)]
pub struct CharacterAffiliation {
    pub alliance_id: u64,
    pub character: u64,
    pub corporation: u64,
}

impl<'a> CharacterGroup<'a> {
    /// Get a character's public information.
    pub async fn get_public_info(
        &self,
        character_id: u64,
    ) -> Result<CharacterPublicInfo, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id")?
            .replace("character_id", &character_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get a character's corporation history.
    pub async fn get_history(
        &self,
        character_id: u64,
    ) -> Result<Vec<CorporationHistoryItem>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id_corporationhistory")?
            .replace("character_id", &character_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get a character's portrait URLs on the image server.
    pub async fn get_portrait(&self, character_id: u64) -> Result<CharacterPortraitInfo, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_characters_character_id_portrait")?
            .replace("character_id", &character_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get character affiliations.
    pub async fn get_affiliation(
        &self,
        character_ids: &[u64],
    ) -> Result<Vec<CharacterAffiliation>, EsiError> {
        let path = self
            .esi
            .get_endpoint_for_op_id("post_characters_affiliation")?;
        let body = serde_json::to_string(character_ids)?;
        self.esi
            .query("POST", RequestType::Public, &path, None, Some(&body))
            .await
    }

    // more endpoints ...
}
