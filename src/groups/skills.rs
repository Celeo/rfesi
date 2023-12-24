use crate::prelude::*;

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Skill {
    pub skill_id: u32,
    pub active_skill_level: u32,
    pub skillpoints_in_skill: u64,
    pub trained_skill_level: u32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Skills {
    pub skills: Vec<Skill>,
    pub total_sp: u64,
    pub unallocated_sp: u32,
}

/// Endpoints for Skills
pub struct SkillsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

impl<'a> SkillsGroup<'a> {
    api_get!(
        /// Get character skills.
        get_skills,
        "get_characters_character_id_skills",
        RequestType::Authenticated,
        Skills,
        (character_id: u64) => "{character_id}"
    );
}
