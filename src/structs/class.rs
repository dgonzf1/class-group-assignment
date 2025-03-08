use std::collections::HashMap;

pub struct Class {
    class_id: u64,
    name: String,
    description: Option<String>,
    professor: String,
    max_students: u16,
    min_students: u16,
    group: u8, // This is to be assigned.
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
