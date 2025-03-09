use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    group: u8, // This is to be assigned.
    #[serde(default)]
    students_inscribed: Vec<u64>,
}

impl Class {
    pub fn inscribe_student(&mut self, student_id: u64) -> Result<(), ()> {
        if self.get_inscribed_students() <= self.max_students {
            self.students_inscribed.push(student_id);
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
