use std::{collections::{BTreeSet, HashMap, HashSet}, fs::File};
use std::io::Write;
use anyhow::{Result, anyhow};
use chrono::{Datelike, Days, Duration, NaiveDate, NaiveTime, Weekday};
use reqwest::Client;
use scraper::{Html, Selector}; // other scraper crate

use shared::{data::{load_main_programs_from_json, load_programs_from_json}, definitions::{ClassroomId, Entry, EntryType, ExerciseType, MainProgramId, ProgramId, Semester, Subject, SubjectId, TeacherId}, time::{DURATION_MAP, START_TIME_MAP}};

use crate::get_semester_data;


pub async fn get_and_write(semesters: &Vec<Semester>) -> Result<()> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; rust-reqwest-scraper/1.0)")
        .build()?;

    let programs = load_programs_from_json()?;

    let mut subjects = HashMap::new();
    let mut entries = vec![];

    let selector = Selector::parse(".entry-absolute-box").unwrap();

    for semester in semesters {
        println!("Semester: {semester:?}");
        let (start_date, end_date) = get_semester_data(semester);
        let mondays = get_mondays_from_to(start_date, end_date)?;

        for program_id in programs.keys() {
            println!("Program id: {program_id:?}");

            for day in &mondays {
                let url =
                    format!("https://urnik.fs.uni-lj.si/layer_one/{}/?day={day}", program_id.0);
                println!("Scraping: {url}");

                let resp = client.get(&url).send().await?.text().await?;
                let document = Html::parse_document(&resp);

                for element in document.select(&selector) {
                    let html_str = element.html();
                    // If return error it will just skip entry
                    let _ = handle_subjects_entries(
                        &html_str,
                        semester,
                        &mut subjects,
                        &mut entries,
                    );
                }
            }
        }
    }

    // Write JSON outputs
    write_subjects_json("static/subjects.json", &subjects)?;
    write_entries_json("static/entries.json", &entries)?;

    Ok(())
}


fn handle_subjects_entries(html_entry_str: &str, semester: &Semester, subjects: &mut HashMap<SubjectId, Subject>, entries: &mut Vec<Entry>) -> Result<()> {
    let fragment = Html::parse_fragment(html_entry_str);

    let entry_type = get_entry_type(&fragment)?;
    let teacher_ids = get_teacher_ids(&fragment)?;
    let classroom_id = get_classroom_id(&fragment)?;
    let (date, start_time, duration) = get_date_start_time_duration(&fragment)?;
    let (subject_id, _subject_full, subject_name, subject_abbr, subject_url) = get_subject_data(&fragment)?;
    let (program_ids, main_program_id) = get_main_programs(&fragment)?;
    match subjects.entry(subject_id) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            match entry_type.clone() {
                EntryType::Lecture => {},
                EntryType::Exercise(type_) => {
                    match type_ {
                        ExerciseType::Tutorial(i) => {entry.get_mut().tutorial_groups.extend(i);},
                        ExerciseType::Lab(i) => {entry.get_mut().lab_groups.extend(i);},
                    }
                }
            }
        }
        std::collections::hash_map::Entry::Vacant(_entry) => {
            let (tut, lab) = match entry_type.clone() {
                EntryType::Lecture => {(BTreeSet::new(), BTreeSet::new())},
                EntryType::Exercise(type_) => {
                    match type_ {
                        ExerciseType::Tutorial(i) => (i.into_iter().collect::<BTreeSet<_>>(), BTreeSet::new()),
                        ExerciseType::Lab(i) => (BTreeSet::new(), i.into_iter().collect::<BTreeSet<_>>()),
                    }
                }
            };
            let subject = Subject {
                id: subject_id,
                program_ids,
                main_program_id,
                semester: *semester,
                name: subject_name,
                abbr: subject_abbr,
                url: subject_url,
                tutorial_groups: tut,
                lab_groups: lab,
            };
            subjects.insert(subject_id, subject);
        }
    };

    let entry = Entry {
        subject_id,
        entry_type,
        teacher_ids,
        classroom_id,
        date,
        week_iso: date.iso_week().week() as i32,
        start_time,
        duration,
    };
    entries.push(entry);

    Ok(())

}

/// Gets all monday dates d1-6days to d2
pub fn get_mondays_from_to(d1: NaiveDate, d2: NaiveDate) -> Result<Vec<NaiveDate>> {
    let mut mondays = Vec::new();
    let mut current = d1
        .checked_sub_signed(Duration::days(6))
        .ok_or_else(|| anyhow!("Date underflow when subtracting 6 days from {d1}"))?;

    // Move current to the first Monday >= current
    while current.weekday() != Weekday::Mon {
        current = current
            .succ_opt()
            .ok_or_else(|| anyhow!("Date overflow while moving to first Monday"))?;
    }

    // Collect all Mondays until d2
    while current <= d2 {
        mondays.push(current);
        current = current
            .checked_add_signed(Duration::days(7))
            .ok_or_else(|| anyhow!("Date overflow while adding 7 days to {current}"))?;
    }

    Ok(mondays)
}


pub fn get_date_start_time_duration(fragment: &Html) -> Result<(NaiveDate, NaiveTime, NaiveTime)> {
    // Parse the entry div
    let style_selector = Selector::parse("div.entry-absolute-box")
        .map_err(|e| anyhow!("Selector parse error: {e:?}"))?;
    let style_div = fragment
        .select(&style_selector)
        .next()
        .ok_or_else(|| anyhow!("No div.entry-absolute-box found"))?;

    let style = style_div
        .value()
        .attr("style")
        .ok_or_else(|| anyhow!("No style attribute found"))?;

    let mut start_time: Option<NaiveTime> = None;
    let mut duration: Option<NaiveTime> = None;
    let mut day_offset: Option<u32> = None;

    for kv in style.split(';') {
        let kv = kv.trim();
        if kv.is_empty() { continue; }
        let mut parts = kv.splitn(2, ':');
        let key = parts.next().ok_or_else(|| anyhow!("Malformed style: {kv}"))?.trim();
        let value = parts.next().ok_or_else(|| anyhow!("Malformed style: {kv}"))?.trim();

        match key {
            "top" => {
                start_time = START_TIME_MAP.get_by_left(value).copied();
                if start_time.is_none() {
                    eprintln!("Invalid top value: {value}");
                }
            }
            "height" => {
                duration = DURATION_MAP.get_by_left(value).copied();
                if duration.is_none() {
                    eprintln!("Invalid height value: {value}");
                }
            }
            "left" => {
                let num: f32 = value
                    .trim_end_matches('%')
                    .parse()
                    .map_err(|e| anyhow!("Invalid left percentage '{value}': {e}"))?;

                day_offset = if (0.0..20.0).contains(&num) {
                    Some(0)
                } else if (20.0..40.0).contains(&num) {
                    Some(1)
                } else if (40.0..60.0).contains(&num) {
                    Some(2)
                } else if (60.0..80.0).contains(&num) {
                    Some(3)
                } else if (80.0..=100.0).contains(&num) {
                    Some(4)
                } else {
                    return Err(anyhow!("Invalid left percentage: {num}"));
                };
            }
            _ => {}
        }
    }

    let start_time = start_time.ok_or_else(|| anyhow!("Cannot determine start time"))?;
    let duration = duration.ok_or_else(|| anyhow!("Cannot determine duration"))?;

    // Get classroom to extract base date
    let selector = Selector::parse("div.classroom a")
        .map_err(|e| anyhow!("Selector parse error: {e:?}"))?;
    let classroom = fragment
        .select(&selector)
        .next()
        .ok_or_else(|| anyhow!("No classroom link found"))?;
    let href = classroom
        .value()
        .attr("href")
        .ok_or_else(|| anyhow!("Missing href attribute"))?;

    // Extract date from href like "/classroom/194/?day=2025-10-05"
    let date_str = href
        .split("?day=")
        .nth(1)
        .ok_or_else(|| anyhow!("No ?day= query in href"))?;
    let mut date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| anyhow!("Invalid date in href ({date_str}): {e}"))?;

    // Apply day offset
    if let Some(offset) = day_offset {
        date = date
            .checked_add_days(Days::new(offset as u64))
            .ok_or_else(|| anyhow!("Date overflow when adding offset {offset}"))?;
    }

    Ok((date, start_time, duration))
}

fn get_classroom_id(fragment: &Html) -> Result<ClassroomId> {
    let classroom_selector = Selector::parse("div.classroom a")
        .map_err(|e| anyhow!("Failed to parse selector: {e:?}"))?;

    let classroom = fragment
        .select(&classroom_selector)
        .next()
        .ok_or_else(|| anyhow!("No classroom link found"))?;

    let href = classroom
        .value()
        .attr("href")
        .ok_or_else(|| anyhow!("Missing href attribute"))?;

    let id_str = href
        .split('/')
        .nth(2)
        .ok_or_else(|| anyhow!("Href has unexpected format: {href}"))?;

    let id = id_str
        .parse::<u32>()
        .map_err(|e| anyhow!("Invalid classroom ID '{id_str}': {e}"))?;

    Ok(ClassroomId(id))
}

fn get_teacher_ids(fragment: &Html) -> Result<HashSet<TeacherId>> {
    let teacher_selector = Selector::parse("div.teacher a")
        .map_err(|e| anyhow!("Failed to parse selector: {e:?}"))?;

    let teacher_ids = fragment
        .select(&teacher_selector)
        .filter_map(|e| {
            e.value()
                .attr("href")
                .and_then(|href| {
                    // href looks like "/teacher/712/?day=2025-10-05"
                    href.split('/')
                        .nth(2)
                        .and_then(|id| id.parse::<u32>().ok())
                        .map(TeacherId)
                })
        })
        .collect();

    Ok(teacher_ids)
}

fn get_entry_type(fragment: &Html) -> Result<EntryType> {
    let entry_type_selector = Selector::parse("span.entry-type")
        .map_err(|e| anyhow!("Failed to parse selector: {e:?}"))?;

    let entry_type_el = fragment
        .select(&entry_type_selector)
        .next()
        .ok_or_else(|| anyhow!("No <span class=\"entry-type\"> element found"))?;

    let entry_type_str = entry_type_el.inner_html();
    let entry_type_str = entry_type_str.trim();

    let entry_type = EntryType::from_string(entry_type_str)?;
    Ok(entry_type)
}

fn get_subject_data(fragment: &Html) -> Result<(SubjectId, String, String, String, String)> {
    let subject_selector = Selector::parse("a.subject")
        .map_err(|e| anyhow!("Failed to parse selector: {e:?}"))?;

    let subject_el = fragment
        .select(&subject_selector)
        .next()
        .ok_or_else(|| anyhow!("No <a class=\"subject\"> element found"))?;

    let subject_url = subject_el
        .value()
        .attr("href")
        .ok_or_else(|| anyhow!("Missing href attribute on subject link"))?
        .to_string();

    let subject_full = subject_el
        .value()
        .attr("title")
        .ok_or_else(|| anyhow!("Missing title attribute on subject link"))?
        .to_string();

    let subject_name = subject_full
        .split('(')
        .next()
        .ok_or_else(|| anyhow!("Invalid subject title format: {subject_full}"))?
        .trim()
        .to_string();

    let subject_abbr = subject_full
        .rfind('(')
        .and_then(|start| {
            subject_full
                .rfind(')')
                .map(|end| subject_full[start + 1..end].to_string())
        })
        .unwrap_or_default();

    let mut parts = subject_url.splitn(2, '?');
    let path = parts.next().ok_or_else(|| anyhow!("Invalid subject URL: {subject_url}"))?;
    let path = path.trim_end_matches('/');

    let id_str = path
        .rsplit('/')
        .next()
        .ok_or_else(|| anyhow!("Invalid subject path: {path}"))?;

    let id = id_str
        .parse::<u32>()
        .map(SubjectId)
        .map_err(|e| anyhow!("Invalid subject ID '{id_str}': {e}"))?;

    Ok((id, subject_full, subject_name, subject_abbr, subject_url))
}

fn get_main_programs(fragment: &Html) -> Result<(BTreeSet<ProgramId>, MainProgramId)> {
    let selector = Selector::parse("span.layer_one a")
        .map_err(|e| anyhow!("Failed to parse selector: {e:?}"))?;

    // Extract all ProgramId values
    let program_ids: BTreeSet<ProgramId> = fragment
        .select(&selector)
        .filter_map(|e| {
            e.value()
                .attr("href")
                .and_then(|href| href.split('/').nth(2)?.parse::<u32>().ok().map(ProgramId))
        })
        .collect();

    // Extract group from the first <a> only
    let group = fragment
        .select(&selector)
        .next()
        .and_then(|e| {
            e.value().attr("title").and_then(|title| {
                let start = title.find('(')? + 1;
                let end = title.find(')')?;
                title.get(start..end).map(|s| s.to_string())
            })
        })
        .ok_or_else(|| anyhow!("No group title found in first <a> element"))?;

    let main_programs = load_main_programs_from_json()?;
    let main_program_id = main_programs
        .into_iter()
        .find(|mp| mp.1.name == group)
        .map(|mp| mp.1.id)
        .ok_or_else(|| anyhow!("No main program with matching name found: {group}"))?;

    Ok((program_ids, main_program_id))
}


pub fn write_subjects_json(file_path: &str, subjects: &HashMap<SubjectId, Subject>) -> std::io::Result<()> {
    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(subjects)
        .expect("Failed to serialize subjects to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn write_entries_json(file_path: &str, entries: &Vec<Entry>) -> std::io::Result<()> {
    // Serialize entries to pretty JSON
    let json_string = serde_json::to_string_pretty(entries)
        .expect("Failed to serialize entries to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
