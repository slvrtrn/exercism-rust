use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grade_to_students: BTreeMap<u32, BTreeSet<String>>
}

impl School {
    pub fn new() -> School {
        Self { grade_to_students: BTreeMap::new() }
    }
    pub fn add(&mut self, grade: u32, student: &str) {
        match self.grade_to_students.get_mut(&grade) {
            None => {
                let mut students = BTreeSet::new();
                students.insert(student.to_owned());
                self.grade_to_students.insert(grade, students);
            }
            Some(students) => {
                students.insert(student.to_owned());
            }
        };
    }
    pub fn grades(&self) -> Vec<u32> {
        self.grade_to_students.keys().map(|x| x.to_owned()).collect()
    }
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grade_to_students.get(&grade) {
            None => vec![],
            Some(students) => students.iter().map(|x| x.to_owned()).collect(),
        }
    }
}
