use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    // TODO: passing id as different types.
    student_id: u64,
    name: String,
    // TODO: Year should give priority for taking classes.
    // for the first iteration, this will not be implemented.
    year: i8,
    class_priority: HashMap<u64, u8>, // class_priority : class_id
    #[serde(default)]
    inscribed_classes: Vec<u64>, // class id
}

impl Student {
    pub fn inscribe_class(&mut self, class_id: u64) -> () {
        self.inscribed_classes.push(class_id);
    }
}

pub fn students_from_json(students_str: String) -> Result<Vec<Student>, String> {
    println!("Entering fn students_from_json");
    let students: Vec<Student> = serde_json::from_str(&students_str).map_err(|e| e.to_string())?;
    println!("{:?}", students[0]);
    Ok(students)
}
