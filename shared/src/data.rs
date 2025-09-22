use std::{collections::HashMap, fs::File, io};
use serde::de::DeserializeOwned;

use crate::definitions::{Classroom, ClassroomId, Entry, MainProgram, MainProgramId, Program, ProgramId, Semester, Subject, SubjectId, Teacher, TeacherId};

fn load_from_json<T>(path: &str) -> io::Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let data = serde_json::from_reader(file)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(data)
}


pub fn load_classrooms_from_json() -> io::Result<HashMap<ClassroomId, Classroom>> {
    load_from_json("static/classrooms.json")
}
pub fn load_entries_from_json() -> io::Result<Vec<Entry>> {
    load_from_json("static/entries.json")
}
pub fn load_main_programs_from_json() -> io::Result<HashMap<MainProgramId, MainProgram>> {
    load_from_json("static/main_programs.json")
}
pub fn load_programs_from_json() -> io::Result<HashMap<ProgramId, Program>> {
    load_from_json("static/programs.json")
}
pub fn load_semesters_from_json() -> io::Result<Vec<Semester>> {
    load_from_json("static/semesters.json")
}
pub fn load_subjects_from_json() -> io::Result<HashMap<SubjectId, Subject>> {
    load_from_json("static/subjects.json")
}
pub fn load_teachers_from_json() -> io::Result<HashMap<TeacherId, Teacher>> {
    load_from_json("static/teachers.json")
}