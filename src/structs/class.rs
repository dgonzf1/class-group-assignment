use serde::{Deserialize, Serialize};

use super::student::Student;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    class_id: u64,
    name: String,
    #[serde(default)]
    description: Option<String>,
    professor: String,
    max_students: u16,
    min_students: u16,
    #[serde(default)]
    group: Option<u8>, // This is to be assigned.
    #[serde(default)]
    students_inscribed: Vec<u64>,
}

impl Class {
    pub fn assign_group(&mut self, group: u8) -> bool {
        if self.group.is_some() {
            // group already assigned.
            return false;
        };
        self.group = Some(group);
        true
    }

    pub fn get_id(&self) -> u64 {
        self.class_id
    }

    pub fn get_professor(&self) -> String {
        self.professor.to_owned()
    }
    pub fn get_students_inscribed(&self) -> Vec<u64> {
        self.students_inscribed.clone()
    }
    pub fn inscribe_student(&mut self, student: &mut Student) -> Result<(), ()> {
        if self.get_inscribed_students() < self.max_students {
            self.students_inscribed.push(student.get_id());
            student.inscribe_class(self.get_id());
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_inscribed_students(&self) -> u16 {
        self.students_inscribed.len() as u16
    }

}

pub fn class_from_json(classes_str: String) -> Result<Vec<Class>, String> {
    println!("Entering fn class_from_json");
    let classes: Vec<Class> = serde_json::from_str(&classes_str).map_err(|e| e.to_string())?;
    println!("{:?}", classes[0]);
    Ok(classes)
}
