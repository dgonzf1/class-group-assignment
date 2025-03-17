mod optimize;
mod structs;

use optimize::assign_class_and_groups;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use structs::class::{Class, class_from_json};
use structs::student::{Student, students_from_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("argument 1: students.json");
    }
    let students_file = &args[1];
    let students_as_str = read_to_string(students_file)?;
    let students = students_from_json(students_as_str)?;
    let mut students_by_id: HashMap<u64, Student> = HashMap::new();
    for student in students {
        students_by_id.insert(student.get_id(), student);
    }

    let class_file = &args[2];
    let class_as_str = read_to_string(class_file)?;
    let classes = class_from_json(class_as_str)?;
    let mut classes_by_id: HashMap<u64, Class> = HashMap::new();
    for class in classes {
        classes_by_id.insert(class.get_id(), class);
    }
    assign_class_and_groups(students_by_id, classes_by_id)?;
    Ok(())
}
