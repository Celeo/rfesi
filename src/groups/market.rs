#![allow(unused)]

use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct HistoryItem {
    pub average: f64,
    pub date: String,
    pub highest: f64,
    pub lowest: f64,
    pub order_count: i64,
    pub volume: i64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct MarketOrder {
    pub duration: i32,
    pub is_buy_order: bool,
    pub issued: String,
    pub location_id: i64,
    pub min_volume: i32,
    pub order_id: i64,
    pub price: f64,
    pub range: String,
    pub system_id: i32,
    pub type_id: i32,
    pub volume_remain: i32,
    pub volume_total: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct PriceItem {
    pub adjusted_price: Option<f64>,
    pub average_price: Option<f64>,
    pub type_id: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterOrder {
    pub duration: i32,
    pub escrow: Option<f64>,
    pub is_buy_order: Option<bool>,
    pub is_corporation: bool,
    pub issued: String,
    pub location_id: i64,
    pub min_volume: Option<i32>,
    pub order_id: i64,
    pub price: f64,
    pub range: String,
    pub region_id: i32,
    pub type_id: i32,
    pub volume_remain: i32,
    pub volume_total: i32,
}

/// Endpoints for Market
pub struct MarketGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> MarketGroup<'a> {
    api_get!(
        /// Get a list of historical market statistics for the specified type in a region
        get_region_history,
        "get_markets_region_id_history",
        RequestType::Public,
        Vec<HistoryItem>,
        (region_id: i32) => "{region_id}";
        (type_id: i32) => "type_id"
    );

    api_get!(
        /// Get a list of orders in a region
        get_region_orders,
        "get_markets_region_id_orders",
        RequestType::Public,
        Vec<MarketOrder>,
        (region_id: i32) => "{region_id}";
        Optional(order_type: String) => "order_type",
        Optional(page: i32) => "page",
        Optional(type_id: i32) => "type_id"
    );

    api_get!(
        /// Get a list of average and adjusted prices
        get_market_prices,
        "get_markets_prices",
        RequestType::Public,
        Vec<PriceItem>,
    );

    api_get!(
        /// List open market orders placed by a character
        get_character_orders,
        "get_characters_character_id_orders",
        RequestType::Authenticated,
        Vec<CharacterOrder>,
        (character_id: u64) => "{character_id}"
    );
}
