use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
    student_grades: BTreeMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
            student_grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(&existing_grade) = self.student_grades.get(student) {
            if existing_grade != grade {
                return;
            }
        }

        self.roster
            .entry(grade)
            .or_default()
            .insert(student.to_string());
        self.student_grades.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .map_or(Vec::new(), |students| students.iter().cloned().collect())
    }
}
