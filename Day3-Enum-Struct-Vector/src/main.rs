use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(String::from(student), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for (_, x) in self.students.iter() {
            result.push(*x);
        }

        result.sort();
        result.dedup();
        result
    }

    pub fn find_student_by_grade(&self, grade: u32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (k, v) in &self.students {
            if *v == grade {
                result.push(k.parse().unwrap());
            }
        }
        result.sort();
        result
    }
}

#[test]
fn init_school() {
    let school: School = School::new();
    assert_eq!(school.students.len(), 0);
}

#[test]
fn add_new_student() {
    // unimplemented!();
    let mut school: School = School::new();
    school.add(10, "Alice");
    school.add(9, "Bob");
    assert_eq!(school.students.len(), 2);
}

#[test]
fn find_students_by_grade() {
    let mut school: School = School::new();
    school.add(10, "Alice");
    school.add(9, "Bob");

    let students = school.find_student_by_grade(10);
    assert_eq!(students[0], "Alice");
}
