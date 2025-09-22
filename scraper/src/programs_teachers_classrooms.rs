use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::Write;
use anyhow::Result;
use reqwest::Client;

use shared::definitions::{Classroom, ClassroomId, MainProgram, MainProgramId, Program, ProgramId, Teacher, TeacherId};

use crate::get_valid_program_ids;


fn extract_index(url: &str) -> Option<u32> {
    url.trim_matches('/')
        .split('/')
        .next_back()
        .and_then(|s| s.parse::<u32>().ok())
}


pub async fn get_and_write() -> Result<()> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; rust-reqwest-scraper/1.0)")
        .build()?;

    let url = "https://urnik.fs.uni-lj.si/layer_one/85/?day=2025-10-23";
    println!("Scraping: {url}");

    let html = client.get(url).send().await?.text().await?;

    let mut main_program_id = 0;
    let mut main_programs: HashMap<String, MainProgram> = HashMap::new();
    let mut programs = Vec::new();
    let mut teachers = Vec::new();
    let mut classrooms = Vec::new();

    for line in html.lines() {
        let line = line.trim();

        if let Some((label, url)) = line.split_once("\": \"") {
            let label = label.trim_matches('"');
            let url = url.trim_end_matches("\",").to_string();

            if let Some(index) = extract_index(&url) {
                if let Some(rest) = label.strip_prefix("Smer: ") {
                    if !get_valid_program_ids().contains(&ProgramId(index)) {
                        continue;
                    }
                    handle_program_main(
                        rest,
                        index,
                        &url,
                        &mut main_programs,
                        &mut main_program_id,
                        &mut programs,
                    );
                } else if let Some(rest) = label.strip_prefix("Učitelj: ") {
                    teachers.push(Teacher {
                        name: rest.to_string(),
                        url,
                        id: TeacherId(index),
                    });
                } else if let Some(rest) = label.strip_prefix("Predavalnica: ") {
                    classrooms.push(Classroom {
                        full_name: rest.to_string(),
                        url,
                        id: ClassroomId(index),
                    });
                }
            }
        }
    }

    let main_programs = main_programs.into_values().map(|v| (v.id, v)).collect();
    write_main_programs_json("static/main_programs.json", &main_programs)?;
    write_programs_json("static/programs.json", &programs)?;
    write_teachers_json("static/teachers.json", &teachers)?;
    write_classrooms_json("static/classrooms.json", &classrooms)?;

    Ok(())
}



fn handle_program_main(rest: &str, index: u32, url: &str, main_programs: &mut HashMap<String, MainProgram>, main_program_id_counter: &mut u32, programs: &mut Vec<Program>) {
    // Save full name
    let full_name = rest.to_string();

    // Extract all parentheses
    let parens = rest.match_indices('(')
        .map(|(start, _)| start)
        .collect::<Vec<_>>();

    let (name, main_program_abbr, abbr) = if parens.len() >= 2 {
        // first '(' index
        let first_start = parens[0];
        let second_start = parens[1];

        let name = rest[..first_start].trim().to_string(); // everything before first '('
        let main_program_abbr = &rest[first_start+1..rest[first_start..].find(')').unwrap_or_else(|| panic!("Can't find first ')' in {rest}")) + first_start]; // content in first ()
        
        // second parentheses
        let second_end = rest[second_start..].find(')').unwrap_or_else(|| panic!("Can't find second ')' in {rest}")) + second_start;
        let abbr = &rest[second_start+1..second_end]; // content in second ()

        (name, main_program_abbr, abbr)
    } else {
        (rest.to_string(), "UNKNOWN", "UNKNOWN")
    };

    // Get or insert MainProgramId
    let main_program_id = match main_programs.entry(main_program_abbr.to_string()) {
        std::collections::hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().program_ids.insert(ProgramId(index));
            entry.get().id
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            let id = MainProgramId(*main_program_id_counter);
            entry.insert(MainProgram {
                id,
                name: main_program_abbr.to_string(),
                program_ids: BTreeSet::from([ProgramId(index)]),
            });
            *main_program_id_counter += 1;
            id
        }
    };

    programs.push(Program {
        id: ProgramId(index),
        full_name,
        name,
        abbr: abbr.to_string(),
        url: url.to_string(),
        main_program_id,
    });
}



pub fn write_main_programs_json(
    file_path: &str,
    main_programs: &HashMap<MainProgramId, MainProgram>,
) -> std::io::Result<()> {
    // Serialize the HashMap to a pretty JSON string
    let json_string = serde_json::to_string_pretty(main_programs)
        .expect("Failed to serialize main programs to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn write_programs_json(file_path: &str, programs: &[Program]) -> std::io::Result<()> {
    // Convert Vec<Program> to HashMap<ProgramId, Program> for easy lookup
    let program_map: std::collections::HashMap<ProgramId, &Program> =
        programs.iter().map(|p| (p.id, p)).collect();

    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(&program_map)
        .expect("Failed to serialize programs to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn write_teachers_json(file_path: &str, teachers: &[Teacher]) -> std::io::Result<()> {
    // Convert Vec<Teacher> into HashMap<TeacherId, Teacher>
    let teacher_map: HashMap<TeacherId, &Teacher> =
        teachers.iter().map(|t| (t.id, t)).collect();

    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(&teacher_map)
        .expect("Failed to serialize teachers to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn write_classrooms_json(file_path: &str, classrooms: &[Classroom]) -> std::io::Result<()> {
    // Convert Vec<Classroom> into HashMap<ClassroomId, Classroom>
    let classroom_map: HashMap<ClassroomId, &Classroom> =
        classrooms.iter().map(|c| (c.id, c)).collect();

    // Serialize to pretty JSON
    let json_string = serde_json::to_string_pretty(&classroom_map)
        .expect("Failed to serialize classrooms to JSON");

    // Write JSON to file
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

