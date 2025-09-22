use shared::db::Database;

pub struct FrontendDatabase;

impl FrontendDatabase {
    pub fn load() -> Database {
        Database {
            main_programs: serde_json::from_str(include_str!("../../static/main_programs.json")).unwrap(),
            programs: serde_json::from_str(include_str!("../../static/programs.json")).unwrap(),
            semesters: serde_json::from_str(include_str!("../../static/semesters.json")).unwrap(),
            subjects: serde_json::from_str(include_str!("../../static/subjects.json")).unwrap(),
            teachers: serde_json::from_str(include_str!("../../static/teachers.json")).unwrap(),
            classrooms: serde_json::from_str(include_str!("../../static/classrooms.json")).unwrap(),
            work_free_days: serde_json::from_str(include_str!("../../static/work_free_days.json")).unwrap(),
            entries: serde_json::from_str(include_str!("../../static/entries.json")).unwrap(),
        }
    }
}