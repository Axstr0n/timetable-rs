use yew::prelude::*;
use std::collections::HashMap;
use chrono::{Datelike, NaiveDate};

use shared::definitions::{MainProgramId, Semester, Subject, SubjectId};

use crate::components::entry_box::entry_to_props;
use crate::components::header::Header;
use crate::components::calendar::Calendar;
use crate::components::subject_abbr_container::SubjectAbbrContainer;
use crate::components::subject_select_container::SubjectContainer;
use crate::components::timetable::Timetable;
use crate::database::FrontendDatabase;
use crate::logic::entries::{arrange_columns, filter_entries};
use crate::logic::visibility::{toggle_group, toggle_subject, SubjectVisibility, SubjectsVisibilityMap};
use crate::logic::date::current_iso_week;
use crate::utils::{create_multiple_colors, Color};




#[function_component(App)]
pub fn app() -> Html {
    
    // region: DATE
    let (iso_year, iso_week) = current_iso_week();
    let current_week = use_state(|| iso_week);
    let current_year = use_state(|| iso_year);

    let on_prev = {
        let current_week = current_week.clone();
        let current_year = current_year.clone();
        Callback::from(move |_| {
            let mut week = *current_week - 1;
            let mut year = *current_year;

            if week < 1 {
                // go to last week of previous year
                year -= 1;
                week = NaiveDate::from_ymd_opt(year, 12, 28)
                    .unwrap()
                    .iso_week()
                    .week() as i32;
            }

            current_week.set(week);
            current_year.set(year);
        })
    };

    let on_next = {
        let current_week = current_week.clone();
        let current_year = current_year.clone();
        Callback::from(move |_| {
            let mut week = *current_week + 1;
            let mut year = *current_year;

            let weeks_in_year = NaiveDate::from_ymd_opt(year, 12, 28)
                .unwrap()
                .iso_week()
                .week() as i32;

            if week > weeks_in_year {
                week = 1;
                year += 1;
            }

            current_week.set(week);
            current_year.set(year);
        })
    };
    // endregion


    let db = FrontendDatabase::load();

    // Options for dropdown
    let mut main_program_options: Vec<(MainProgramId, String)> = db.main_programs
        .iter()
        .map(|(id, program)| (*id, program.name.clone()))
        .collect();
    main_program_options.sort_by_key(|(id, _)| id.0);

    let mut semesters_options: Vec<(Semester, String)> = db.semesters
        .iter()
        .map(|semester| (*semester, format!("{} semester", semester.0).to_string()))
        .collect();
    semesters_options.sort_by_key(|(id, _)| id.0);


    // Make subject visibility map
    let mut subjects: Vec<Subject> = db.subjects.values().cloned().collect();
    subjects.sort_by_key(|s| s.id.0);
    let initial_map: SubjectsVisibilityMap = subjects
        .iter()
        .map(|s| (s.id, SubjectVisibility::new(s)))
        .collect();
    let visibility_map = use_state(|| initial_map);
    drop(subjects);

    // Toggle callbacks
    let on_toggle_subject = {
        let visibility_map = visibility_map.clone();
        Callback::from(move |subject_id: SubjectId| {
            visibility_map.set(toggle_subject((*visibility_map).clone(), subject_id));
        })
    };
    let on_toggle_subject_group = {
        let visibility_map = visibility_map.clone();
        Callback::from(move |(sid, gtype, gid)| {
            visibility_map.set(toggle_group((*visibility_map).clone(), sid, gtype, gid));
        })
    };
    
    // Main program select
    let selected_main_program = use_state(|| MainProgramId(0));
    let on_main_program_change = {
        let selected_main_program = selected_main_program.clone();
        Callback::from(move |program_id: MainProgramId| {
            selected_main_program.set(program_id);
        })
    };

    // Semester select
    let selected_semester = use_state(|| Semester(1));
    let on_semester_change = {
        let selected_semester = selected_semester.clone();
        Callback::from(move |semester_id: Semester| {
            selected_semester.set(semester_id);
        })
    };

    // Valid subjects and subject options
    let mut subjects = db.subjects_for_main_program_and_semester(&selected_main_program, *selected_semester);
    subjects.sort_by_key(|s| s.id.0);
    let mut subject_options: Vec<(SubjectId, String)> = subjects
        .iter()
        .map(|s| (s.id, s.abbr.clone()))
        .collect();
    subject_options.sort_by_key(|(subject_id, _)| *subject_id);
    
    let colors = create_multiple_colors(subject_options.len());
    let subject_colors: HashMap<SubjectId, Color> = subject_options
        .iter()
        .map(|(subject_id, _name)| *subject_id)
        .zip(colors.into_iter())
        .collect();

    
    // Entries
    let entries = db.query()
        .main_program(&selected_main_program)
        .semester(*selected_semester)
        .week(*current_week)
        .run()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    let entry_props = entries
        .iter()
        .map(|e| entry_to_props(&db, e))
        .collect::<Vec<_>>();

    let entry_props = filter_entries(entry_props, &visibility_map);
    let mut entry_props = arrange_columns(entry_props);

    for entry in entry_props.iter_mut() {
        entry.color = subject_colors.get(&entry.subject_id).unwrap().clone();
    }

    let all_wfd: Vec<NaiveDate> = db.work_free_days;
    let wfd: Vec<NaiveDate> = all_wfd
        .into_iter()
        .filter(|d| {
            let iso = d.iso_week();
            iso.year() == *current_year && iso.week() == *current_week as u32
        })
        .collect();


    html! {
        <>
            <Header
                main_program_options={main_program_options.clone()}
                selected_main_program={*selected_main_program}
                on_main_program_change={on_main_program_change}

                semester_options={semesters_options.clone()}
                selected_semester={*selected_semester}
                on_semester_change={on_semester_change}
            />
            <SubjectAbbrContainer
                subjects_visibility={(*visibility_map).clone()} 
                subjects={subject_options}
                subject_colors={subject_colors.clone()}
                on_toggle={on_toggle_subject.clone()}
            />
            <SubjectContainer
                subjects={subjects}
                subjects_visibility={(*visibility_map).clone()}
                subject_colors={subject_colors}
                on_toggle_subject={on_toggle_subject}
                on_toggle_group={on_toggle_subject_group}
            />
            <Calendar
                current_week = {*current_week}
                year = {*current_year}
                on_previous = {on_prev}
                on_next = {on_next}
            />
            <Timetable
                current_week = {*current_week}
                year = {*current_year}
                entries={entry_props}
                work_free_days={wfd}
            />
        </>
    }
}
