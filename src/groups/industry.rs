#![allow(unused)]
#![allow(missing_docs)]

use crate::prelude::*;

/// Endpoints for Industry
pub struct IndustryGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct CostIndex {
    pub activity: String,
    pub cost_index: f32
}

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use log::{debug, info};
    use crate::prelude::{EsiBuilder, EsiError};

    #[tokio::test]
    async fn test_get_industry_systems() -> Result<(), Box<dyn std::error::Error>> {
        let mut esi = EsiBuilder::new()
        .user_agent("github.com/celeo/rfesi :: tests :: industry_systems").build()?;
        esi.update_spec().await?;
        let systems = esi.group_industry().get_industry_systems().await?;
        info!("Found {} systems", systems.len());
        Ok(())
    }
}