use std::fs::File;
use std::io::Write;
use chrono::NaiveDate;
use anyhow::Result;

use shared::definitions::{ProgramId, Semester};

pub mod programs_teachers_classrooms;
pub mod work_free_days;
pub mod subjects_entries;


pub fn get_semesters() -> Vec<Semester> {
    vec![
        Semester(1),
        Semester(2),
    ]
}

pub fn get_semester_data(semester: &Semester) -> (NaiveDate, NaiveDate) {
    match semester {
        Semester(1) => {(
            NaiveDate::parse_from_str("2025-10-01", "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str("2026-01-26", "%Y-%m-%d").unwrap(),
        )},
        Semester(2) => {(
            NaiveDate::parse_from_str("2026-02-09", "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str("2026-06-01", "%Y-%m-%d").unwrap(),
        )},
        _ => panic!("Invalid semester {semester:?}"),
    }
}

/// Valid program ids because some on website are outdated
pub fn get_valid_program_ids() -> Vec<ProgramId> {
    let valid_ids: Vec<u32> = vec![
        // 65, // PAP-1
        // 66, 67, 68, 69, 70, 71, 74, 72, 73, // PAP-2
        // 101, 106, 104, 103, 107, 102, 105, 106, 109, // PAP-3
        80, // RRP-1
        81, // RRP-2
        82, // RRP-3
        85, 86, 83, 87, 84, 88, // MAG-1
        91, 92, 89, 93, 90, 94 // MAG-2
    ];
    valid_ids.iter().map(|id| ProgramId(*id)).collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = std::time::Instant::now();

    let semesters = get_semesters();
    write_semesters_json("static/semesters.json", &semesters)?;

    work_free_days::get_and_write(&semesters).await?;
    programs_teachers_classrooms::get_and_write().await?;
    subjects_entries::get_and_write(&semesters).await?;

    let duration = start.elapsed();
    println!("✅ Finished in {duration:?}");

    Ok(())
}


pub fn write_semesters_json(file_path: &str, semesters: &[Semester]) -> std::io::Result<()> {

    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(&semesters)
        .expect("Failed to serialize semesters to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
