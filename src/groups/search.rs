#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Search
pub struct SearchGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> SearchGroup<'a> {
    api_get!(
        /// Search for entities that match a given sub-string.
        search,
        "get_characters_character_id_search",
        RequestType::Authenticated,
        SearchResult,
        (character_id: i32) => "{character_id}";
        (categories: String) => "categories",
        (search: String) => "search";
        Optional(strict: bool) => "strict"
    );
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct SearchResult {
    pub agent: Option<Vec<i32>>,
    pub alliance: Option<Vec<i32>>,
    pub character: Option<Vec<i32>>,
    pub constellation: Option<Vec<i32>>,
    pub corporation: Option<Vec<i32>>,
    pub faction: Option<Vec<i32>>,
    pub inventory_type: Option<Vec<i32>>,
    pub region: Option<Vec<i32>>,
    pub solar_system: Option<Vec<i32>>,
    pub station: Option<Vec<i32>>,
    pub structure: Option<Vec<u64>>,
}
