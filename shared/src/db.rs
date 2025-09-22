use std::collections::HashMap;
use chrono::{Datelike, NaiveDate};
use serde::{Serialize};

use crate::definitions::{Classroom, ClassroomId, Entry, MainProgram, MainProgramId, Program, ProgramId, Semester, Subject, SubjectId, Teacher, TeacherId};


#[derive(Debug, Serialize)]
pub struct Database {
    pub main_programs: HashMap<MainProgramId, MainProgram>,
    pub programs: HashMap<ProgramId, Program>,
    pub semesters: Vec<Semester>,
    pub subjects: HashMap<SubjectId, Subject>,
    pub teachers: HashMap<TeacherId, Teacher>,
    pub classrooms: HashMap<ClassroomId, Classroom>,
    pub work_free_days: Vec<NaiveDate>,
    pub entries: Vec<Entry>,
}
impl Database {
    pub fn query(&self) -> Query {
        Query {
            db: self,
            main_program: None,
            program: None,
            subject: None,
            teacher: None,
            classroom: None,
            semester: None,
            date: None,
            week: None,
            year: None,
        }
    }
    pub fn subjects_for_main_program_and_semester(
        &self,
        main_program_id: &MainProgramId,
        semester: Semester,
    ) -> Vec<Subject> {
        self.subjects
            .values()
            .filter(|subject| subject.main_program_id == *main_program_id && subject.semester == semester)
            .cloned()
            .collect()
    }

}


pub struct Query<'a> {
    db: &'a Database,
    main_program: Option<&'a MainProgramId>,
    program: Option<&'a ProgramId>,
    subject: Option<&'a SubjectId>,
    teacher: Option<&'a TeacherId>,
    classroom: Option<&'a ClassroomId>,
    semester: Option<Semester>,
    date: Option<NaiveDate>,
    week: Option<i32>,
    year: Option<i32>,
}

impl<'a> Query<'a> {
    pub fn main_program(mut self, id: &'a MainProgramId) -> Self {
        self.main_program = Some(id);
        self
    }

    pub fn program(mut self, id: &'a ProgramId) -> Self {
        self.program = Some(id);
        self
    }

    pub fn subject(mut self, id: &'a SubjectId) -> Self {
        self.subject = Some(id);
        self
    }

    pub fn teacher(mut self, id: &'a TeacherId) -> Self {
        self.teacher = Some(id);
        self
    }

    pub fn classroom(mut self, id: &'a ClassroomId) -> Self {
        self.classroom = Some(id);
        self
    }

    pub fn semester(mut self, sem: Semester) -> Self {
        self.semester = Some(sem);
        self
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn week(mut self, week: i32) -> Self {
        self.week = Some(week);
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn run(self) -> Vec<&'a Entry> {
        self.db.entries.iter().filter(|entry| {
            // Filter by subject if requested
            if let Some(subject_id) = self.subject {
                if &entry.subject_id != subject_id {
                    return false;
                }
            }

            // Filter by teacher
            if let Some(teacher_id) = self.teacher {
                if !entry.teacher_ids.contains(teacher_id) {
                    return false;
                }
            }

            // Filter by classroom
            if let Some(classroom_id) = self.classroom {
                if &entry.classroom_id != classroom_id {
                    return false;
                }
            }

            // Filter by date
            if let Some(date) = self.date {
                if entry.date != date {
                    return false;
                }
            }

            // Filter by week
            if let Some(week) = self.week {
                if entry.week_iso != week {
                    return false;
                }
            }

            // Filter by year
            if let Some(year) = self.year {
                if entry.date.year() != year {
                    return false;
                }
            }

            // Subject-based filters require looking up in `subjects`
            if let Some(subject) = self.db.subjects.get(&entry.subject_id) {
                // Semester filter
                if let Some(sem) = self.semester {
                    if subject.semester != sem {
                        return false;
                    }
                }

                // Main program filter
                if let Some(mp_id) = self.main_program {
                    if &subject.main_program_id != mp_id {
                        return false;
                    }
                }

                // Program filter
                if let Some(p_id) = self.program {
                    if !&subject.program_ids.contains(p_id) {
                        return false;
                    }
                }
            } else {
                return false; // subject missing in DB
            }

            true
        }).collect()
    }
}
