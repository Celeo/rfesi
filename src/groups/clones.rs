use crate::prelude::*;

/// Endpoints for Clones
pub struct ClonesGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CloneHome {
    pub location_id: Option<u64>,
    pub location_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct JumpClone {
    pub implants: Vec<u32>,
    pub jump_clone_id: u32,
    pub location_id: u64,
    pub location_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Clones {
    pub home_location: Option<CloneHome>,
    pub jump_clones: Vec<JumpClone>,
    pub last_clone_jump_date: String,
}

impl<'a> ClonesGroup<'a> {
    api_get!(
        /// Get a character's clones.
        get_clones,
        "get_characters_character_id_clones",
        RequestType::Authenticated,
        Clones,
        (character_id: u64) => "{character_id}"
    );

    api_get!(
        /// Get a character's (active clone's) implants.
        get_clone_implants,
        "get_characters_character_id_implants",
        RequestType::Authenticated,
        Vec<u32>,
        (character_id: u64) => "{character_id}"
    );
}
