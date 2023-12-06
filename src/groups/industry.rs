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
    activity_id: i32,
    blueprint_id: i64,
    blueprint_location_id: i64,
    blueprint_type_id: i32,
    completed_character_id: Option<i32>,
    completed_date: Option<String>,
    cost: Option<f64>,
    duration: i32,
    end_date: String,
    facility_id: i64,
    installer_id: i32,
    job_id: i32,
    licensed_runs: Option<i32>,
    output_location_id: i64,
    pause_date: Option<String>,
    probability: Option<f64>,
    product_type_id: Option<i32>,
    runs: i32,
    start_date: String,
    station_id: i64,
    status: String,
    successful_runs: Option<i32>,
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
