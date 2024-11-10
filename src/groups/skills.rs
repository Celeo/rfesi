use crate::prelude::*;

/// Endpoints for Skills
pub struct SkillsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Skill {
    pub skill_id: i32,
    pub active_skill_level: i32,
    pub skillpoints_in_skill: i64,
    pub trained_skill_level: i32,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Skills {
    pub skills: Vec<Skill>,
    pub total_sp: i64,
    pub unallocated_sp: i32,
}

impl SkillsGroup<'_> {
    api_get!(
        /// Get character skills.
        get_skills,
        "get_characters_character_id_skills",
        RequestType::Authenticated,
        Skills,
        (character_id: i32) => "{character_id}"
    );
}
