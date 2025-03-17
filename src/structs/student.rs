use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    // TODO: passing id as different types.
    student_id: u64,
    name: String,
    // TODO: Year should give priority for taking classes.
    // for the first iteration, this will not be implemented.
    year: i8,
    class_priority: HashMap<u8, u64>, // class_priority : class_id
    #[serde(default)]
    inscribed_classes: HashSet<u64>, // class id

    // this variable tell in which preference are the students right now
    #[serde(default = "default_current_preference")]
    current_preference: u8,
}

fn default_current_preference() -> u8 {
    1
}
impl Student {
    pub fn get_next_class_priority_from_group(&self, class_in_group: HashSet<u64>) -> Option<u64> {
        let mut priority: u8 = 1;
        let max_possible_priority = self.class_priority.len() + 1;
        while priority < max_possible_priority as u8 {
            if let Some(class_id) = self.class_priority.get(&priority) {
                if class_in_group.contains(class_id) {
                    return Some(*class_id);
                }
            }
            priority += 1;
        }
        None
    }
    pub fn inscribe_class(&mut self, class_id: u64) -> bool {
        // TODO : This limit should be a constant, and not be set here
        if self.inscribed_classes.len() < 3 {
            for (priority, class_priority_id) in &self.class_priority {
                if *class_priority_id == class_id {
                    self.current_preference = *priority;
                }
            }
            self.inscribed_classes.insert(class_id)
        } else {
            false
        }
    }

    pub fn unsuscribe_class(&mut self, class_id: u64) -> bool {
        self.current_preference += 1;
        self.inscribed_classes.remove(&class_id)
    }

    pub fn get_class_id_by_priority(&self, priority: u8) -> u64 {
        match self.class_priority.get(&priority) {
            Some(priority) => *priority,
            None => 0,
        }
    }
    pub fn get_id(&self) -> u64 {
        self.student_id
    }
}

pub fn students_from_json(students_str: String) -> Result<Vec<Student>, String> {
    println!("Entering fn students_from_json");
    let students: Vec<Student> = serde_json::from_str(&students_str).map_err(|e| e.to_string())?;
    println!("{:?}", students[0]);
    Ok(students)
}
