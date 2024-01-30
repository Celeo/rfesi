use crate::prelude::*;

/// Endpoints for Clones
pub struct ClonesGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CloneHome {
    pub location_id: Option<i64>,
    pub location_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct JumpClone {
    pub implants: Vec<i32>,
    pub jump_clone_id: i32,
    pub location_id: i64,
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
        (character_id: i32) => "{character_id}"
    );

    api_get!(
        /// Get a character's (active clone's) implants.
        get_clone_implants,
        "get_characters_character_id_implants",
        RequestType::Authenticated,
        Vec<u32>,
        (character_id: i32) => "{character_id}"
    );
}
