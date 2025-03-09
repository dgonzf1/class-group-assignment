mod optimize;
mod structs;

use structs::class::class_from_json;

use crate::structs::student::students_from_json;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("argument 1: students.json");
    }
    let students_file = &args[1];
    let students_as_str = if let Ok(students_as_str) = read_to_string(students_file) {
        students_as_str
    } else {
        println!("Error reading students json file");
        return;
    };

    let class_file = &args[2];
    let class_as_str = if let Ok(class_as_str) = read_to_string(class_file) {
        class_as_str
    } else {
        println!("Error reading classes json file");
        return;
    };
    let mut students = students_from_json(students_as_str);
    let mut classes = class_from_json(class_as_str);
    println!("Hello, world!");
}
