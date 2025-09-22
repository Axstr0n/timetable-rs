use std::collections::{BTreeSet, HashSet};
use anyhow::Result;
use crate::anyhow;
use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};

// Unique identifiers
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct MainProgramId(pub u32);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct ProgramId(pub u32);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TeacherId(pub u32);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct ClassroomId(pub u32);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct SubjectId(pub u32);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Semester(pub u8);


// Structs with data

/// PAP-1, RRP-3, MAG-2, ...
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct MainProgram {
    pub id: MainProgramId,
    pub name: String,
    pub program_ids: BTreeSet<ProgramId>,
}

/// (MAG-2) (PCS-MAG), (PAP-2) (PPL-PAP), ...
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Program {
    pub id: ProgramId,
    pub name: String,
    pub full_name: String,
    pub abbr: String,
    pub url: String,
    pub main_program_id: MainProgramId,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Teacher {
    pub id: TeacherId,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Classroom {
    pub id: ClassroomId,
    pub full_name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct Subject {
    pub id: SubjectId,
    pub program_ids: BTreeSet<ProgramId>,
    pub main_program_id: MainProgramId,
    pub semester: Semester,
    pub name: String,
    pub abbr: String,
    pub url: String,
    pub tutorial_groups: BTreeSet<u32>,
    pub lab_groups: BTreeSet<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entry {
    pub subject_id: SubjectId,
    pub entry_type: EntryType,
    pub teacher_ids: HashSet<TeacherId>,
    pub classroom_id: ClassroomId,
    pub date: NaiveDate,
    pub week_iso: i32,
    pub start_time: NaiveTime,
    pub duration: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EntryType {
    Lecture,
    Exercise(ExerciseType),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ExerciseType {
    Tutorial(Vec<u32>),
    Lab(Vec<u32>),
}

impl EntryType {
    pub fn from_string(s: &str) -> Result<EntryType> {
        let s = s.trim();
    
        if s == "P" {
            Ok(EntryType::Lecture)
        } else if s == "VP" {
            Ok(EntryType::Exercise(ExerciseType::Tutorial(vec![])))
        } else if s == "VL" {
            Ok(EntryType::Exercise(ExerciseType::Lab(vec![])))
        } else if s.starts_with("VP(") && s.ends_with(")") {
            // Tutorial
            let inner = &s[3..s.len()-1]; // remove "VL(" and ")"
            let groups: Vec<u32> = inner
                .split(',')
                .filter_map(|g| g.trim().trim_start_matches('S').parse::<u32>().ok())
                .collect();
            Ok(EntryType::Exercise(ExerciseType::Tutorial(groups)))
        } else if s.starts_with("VL(") && s.ends_with(")") {
            // Lab
            let inner = &s[3..s.len()-1];
            let groups: Vec<u32> = inner
                .split(',')
                .filter_map(|g| g.trim().trim_start_matches('S').parse::<u32>().ok())
                .collect();
            Ok(EntryType::Exercise(ExerciseType::Lab(groups)))
        } else {
            Err(anyhow!("Cannot parse '{}' into EntryType", s))
        }
    }
}
impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Lecture => write!(f, "P"),
            EntryType::Exercise(ex) => match ex {
                ExerciseType::Tutorial(groups) => {
                    if groups.is_empty() {
                        write!(f, "VP")
                    } else {
                        let list = groups.iter()
                            .map(|g| format!("S{g}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(f, "VP({list})")
                    }
                }
                ExerciseType::Lab(groups) => {
                    if groups.is_empty() {
                        write!(f, "VL")
                    } else {
                        let list = groups.iter()
                            .map(|g| format!("S{g}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(f, "VL({list})")
                    }
                }
            },
        }
    }
}
