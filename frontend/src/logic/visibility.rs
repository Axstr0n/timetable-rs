use std::collections::BTreeMap;

use shared::definitions::{Subject, SubjectId};
use crate::components::subject_select_container::GroupType;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubjectVisibility {
    pub can_show: bool, // visibility of the whole subject
    pub tutorial_groups: BTreeMap<u32, bool>, // group ID -> can_show
    pub lab_groups: BTreeMap<u32, bool>,      // group ID -> can_show
}

impl SubjectVisibility {
    /// Initialize from a Subject
    pub fn new(subject: &Subject) -> Self {
        let tutorial_groups = subject
            .tutorial_groups
            .iter()
            .map(|&id| (id, true))
            .collect();

        let lab_groups = subject
            .lab_groups
            .iter()
            .map(|&id| (id, true))
            .collect();

        SubjectVisibility {
            can_show: false,
            tutorial_groups,
            lab_groups,
        }
    }

    pub fn set_subject_visibility(&mut self, visible: bool) {
        self.can_show = visible;
    }

    pub fn set_tutorial_group_visibility(&mut self, group_id: u32, visible: bool) {
        if let Some(group) = self.tutorial_groups.get_mut(&group_id) {
            *group = visible;
        }
    }

    pub fn set_lab_group_visibility(&mut self, group_id: u32, visible: bool) {
        if let Some(group) = self.lab_groups.get_mut(&group_id) {
            *group = visible;
        }
    }
}

// Central visibility map
pub type SubjectsVisibilityMap = BTreeMap<SubjectId, SubjectVisibility>;


pub fn toggle_subject(mut map: SubjectsVisibilityMap, subject_id: SubjectId) -> SubjectsVisibilityMap {
    if let Some(v) = map.get_mut(&subject_id) {
        v.can_show = !v.can_show;
    }
    map
}

pub fn toggle_group(
    mut map: SubjectsVisibilityMap,
    subject_id: SubjectId,
    group_type: GroupType,
    group_id: u32,
) -> SubjectsVisibilityMap {
    if let Some(v) = map.get_mut(&subject_id) {
        match group_type {
            GroupType::Tutorial => {
                if let Some(val) = v.tutorial_groups.get_mut(&group_id) {
                    *val = !*val;
                }
            }
            GroupType::Lab => {
                if let Some(val) = v.lab_groups.get_mut(&group_id) {
                    *val = !*val;
                }
            }
        }
    }
    map
}
