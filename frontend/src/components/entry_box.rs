use chrono::{NaiveDate, NaiveTime, Weekday, Datelike};
use shared::{db::{Database}, definitions::{Entry, EntryType, SubjectId}, time::{DURATION_MAP, START_TIME_MAP}};
use yew::prelude::*;

use crate::utils::Color;

pub fn entry_to_props(db: &Database, entry: &Entry) -> EntryBoxProps {
    // Look up the subject
    let subject = db.subjects.get(&entry.subject_id).expect("Subject not found");
    
    // Subject info
    let subject_name = subject.name.clone();
    let subject_abbr = subject.abbr.clone();

    // Entry type
    let entry_type = entry.entry_type.clone();

    // Teachers: convert IDs to names
    let teachers: Vec<String> = entry.teacher_ids.iter()
        .filter_map(|tid| db.teachers.get(tid))
        .map(|t| t.name.clone())
        .collect();

    // Classroom
    let classroom = db.classrooms.get(&entry.classroom_id)
        .map(|c| c.full_name.clone())
        .unwrap_or_else(|| "Unknown".into());

    // Day of week
    let day = entry.date.weekday();

    EntryBoxProps {
        subject_id: entry.subject_id,
        subject_name,
        subject_abbr,
        entry_type,
        teachers,
        classroom,
        day,
        date: entry.date,
        start_time: entry.start_time,
        duration: entry.duration,
        offset_x_multiplier: 0.0, // will be changed later
        total_columns: 1, // will be changed later
        color: Color { r: 0, g: 0, b: 0 } // will be changed later
    }
}


#[derive(Properties, PartialEq, Clone)]
pub struct EntryBoxProps {
    pub subject_id: SubjectId,
    pub subject_name: String,
    pub subject_abbr: String,
    pub entry_type: EntryType,
    pub teachers: Vec<String>,
    pub classroom: String,
    pub day: Weekday,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration: NaiveTime,
    pub offset_x_multiplier: f32,
    pub total_columns: usize,
    pub color: Color,
}

#[function_component(EntryBox)]
pub fn entry_box(props: &EntryBoxProps) -> Html {
    let x_gap = 0.3;
    let y_gap = 0.5;
    let base_width = 20.0;
    let column_width = base_width / props.total_columns as f32;
    let width = column_width - x_gap * 2.0;

    let mut left = match props.day {
        Weekday::Mon => 0.0,
        Weekday::Tue => 20.0,
        Weekday::Wed => 40.0,
        Weekday::Thu => 60.0,
        Weekday::Fri => 80.0,
        _ => {
            web_sys::console::log_1(&"Unknown day".into());
            0.0
        }
    };
    left += props.offset_x_multiplier * column_width + x_gap;

    let top= START_TIME_MAP.get_by_right(&props.start_time).unwrap().trim_end_matches('%').parse::<f32>().unwrap() + y_gap;
    let height = DURATION_MAP.get_by_right(&props.duration).unwrap().trim_end_matches('%').parse::<f32>().unwrap() - y_gap * 3.0;

    let mut font_size = 12;
    if width < 10.0 { font_size = 10; }
    if width < 5.0 { font_size = 9; }
    if width < 3.0 { font_size = 8; }
    if width < 2.0 { font_size = 7; }

    let entry_class = if props.entry_type == EntryType::Lecture {
        "lecture"
    } else {
        "exercise"
    };

    let style_top_height_left_width = format!(
        "top:{top}%; height:{height}%; left:{left}%; width:{width}%; background-color:{}",
        props.color.css()
    );

    let font_style = format!("font-size:{font_size}px");

    let subject_text = if props.subject_name.len() < 25 && width > 9.0 {
        &props.subject_name
    } else {
        &props.subject_abbr
    };

    let entry_type_str = props.entry_type.to_string();
    let show_classroom = width >= 9.0;

    html! {
        <div class={classes!("entry-box", entry_class)} style={style_top_height_left_width}>
            <div class="entry" style={font_style}>
                <div class="subject-title-type-group">
                    <div class="subject-title">{ subject_text }</div>
                    <div class="subject-group">{ entry_type_str }</div>
                </div>
                // <div class="subject-classroom">{ &props.classroom }</div>
                {
                    if show_classroom {
                        html! { <div class="subject-classroom">{ &props.classroom }</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}