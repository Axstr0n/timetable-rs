use std::collections::HashMap;
use chrono::{Timelike, Weekday};
use shared::definitions::{EntryType, ExerciseType};
use crate::{components::entry_box::EntryBoxProps, logic::visibility::SubjectsVisibilityMap};

pub fn filter_entries(
    entries: Vec<EntryBoxProps>,
    visibility_map: &SubjectsVisibilityMap,
) -> Vec<EntryBoxProps> {
    entries.into_iter().filter(|entry| {
        visibility_map.get(&entry.subject_id).is_some_and(|v| {
            if !v.can_show {
                return false;
            }

            match &entry.entry_type {
                EntryType::Lecture => true,
                EntryType::Exercise(ex_type) => match ex_type {
                    ExerciseType::Tutorial(groups) =>
                        groups.iter().any(|gid| v.tutorial_groups.get(gid).copied().unwrap_or(false)),
                    ExerciseType::Lab(groups) =>
                        groups.iter().any(|gid| v.lab_groups.get(gid).copied().unwrap_or(false)),
                },
            }
        })

    }).collect()
}

pub fn arrange_columns(entries: Vec<EntryBoxProps>) -> Vec<EntryBoxProps> {
    let mut lessons_by_day: HashMap<Weekday, Vec<EntryBoxProps>> = HashMap::new();
    for entry in entries {
        lessons_by_day.entry(entry.day).or_default().push(entry);
    }

    let mut final_entries = Vec::new();
    for (_day, day_lessons) in lessons_by_day {
        let mut columns: Vec<Vec<EntryBoxProps>> = Vec::new();

        for lesson in day_lessons {
            let start = lesson.start_time.hour() * 60 + lesson.start_time.minute();
            let end = start + lesson.duration.hour() * 60 + lesson.duration.minute();

            let mut placed = false;
            for col in columns.iter_mut() {
                if col.iter().all(|l| {
                    let l_start = l.start_time.hour() * 60 + l.start_time.minute();
                    let l_end = l_start + l.duration.hour() * 60 + l.duration.minute();
                    end <= l_start || start >= l_end
                }) {
                    col.push(lesson.clone());
                    placed = true;
                    break;
                }
            }
            if !placed {
                columns.push(vec![lesson.clone()]);
            }
        }

        let num_columns = columns.len();
        for (col_idx, col) in columns.iter().enumerate() {
            for lesson in col {
                let mut l = lesson.clone();
                l.offset_x_multiplier = col_idx as f32;
                l.total_columns = num_columns;
                final_entries.push(l);
            }
        }
    }
    final_entries
}
