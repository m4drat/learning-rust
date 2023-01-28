use std::collections::{BTreeMap, BTreeSet};

pub struct School<'a> {
    grades: BTreeMap<u32, BTreeSet<&'a str>>,
}

impl<'a> Default for School<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let entry = self.grades.entry(grade).or_insert(BTreeSet::new());
        entry.insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.iter().map(|(k, _)| *k).collect::<Vec<_>>()
    }

    pub fn grade(&self, grade: u32) -> Vec<&'a str> {
        if let Some(entry) = self.grades.get(&grade) {
            return entry.iter().copied().collect();
        }
        Vec::new()
    }
}
