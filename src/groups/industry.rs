#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Industry
pub struct IndustryGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct CostIndex {
    pub activity: String,
    pub cost_index: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct IndustrialSystem {
    pub cost_indices: Vec<CostIndex>,
    pub solar_system_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct IndustryJob {
    pub activity_id: i32,
    pub blueprint_id: i64,
    pub blueprint_location_id: i64,
    pub blueprint_type_id: i32,
    pub completed_character_id: Option<i32>,
    pub completed_date: Option<String>,
    pub cost: Option<f64>,
    pub duration: i32,
    pub end_date: String,
    pub facility_id: i64,
    pub installer_id: i32,
    pub job_id: i32,
    pub licensed_runs: Option<i32>,
    pub output_location_id: i64,
    pub pause_date: Option<String>,
    pub probability: Option<f64>,
    pub product_type_id: Option<i32>,
    pub runs: i32,
    pub start_date: String,
    pub station_id: i64,
    pub status: String,
    pub successful_runs: Option<i32>,
}

impl<'a> IndustryGroup<'a> {
    api_get!(
        /// Returns a list of solar systems with the cost index for every
        /// activity
        get_industry_systems,
        "get_industry_systems",
        RequestType::Public,
        Vec<IndustrialSystem>,
    );

    api_get!(
        /// List industry jobs placed by a character
        get_character_industry_jobs,
        "get_characters_character_id_industry_jobs",
        RequestType::Authenticated,
        Vec<IndustryJob>,
        (character_id: i32) => "{character_id}"
    );
}
