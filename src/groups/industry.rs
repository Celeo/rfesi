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

impl<'a> IndustryGroup<'a> {
    api_get!(
        /// Returns a list of solar systems with the cost index for every
        /// activity
        get_industry_systems,
        "get_industry_systems",
        RequestType::Public,
        Vec<IndustrialSystem>,
    );
}
