use crate::structs::class::Class;
use crate::structs::student::Student;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Default)]
struct ClassGroups {
    group_id: u8,
    pub students_in_group: HashSet<u64>,
    pub class_in_group: HashSet<u64>,
    pub professor_in_group: HashSet<String>,
}

impl ClassGroups {
    pub fn add_student_to_class(&mut self, student: &mut Student, class: &mut Class) -> bool {
        // check if class in group
        if self.class_in_group.contains(&class.get_id()) {
            return false;
        }

        // add students to group
        if !self.students_in_group.insert(student.get_id()) {
            return false;
        }

        // inscribe student to class
        if class.inscribe_student(student).is_err() {
            return false;
        };

        // add student to students in grou
        true
    }
    pub fn classes_in_group(&self) -> HashSet<u64> {
        self.class_in_group.clone()
    }

    pub fn student_in_group(&self, student: &Student) -> bool {
        self.students_in_group.contains(&student.get_id())
    }
    pub fn add_class_to_group(
        &mut self,
        class: &mut Class,
        students: &mut HashMap<u64, Student>,
    ) -> bool {
        // add professor of class to group
        if !self.professor_in_group.insert(class.get_professor()) {
            // if professor already in group, return false since
            // you can't add another class in this group.
            return false;
        }

        // after checking that the class can be added to this group,
        // add it
        if !self.class_in_group.insert(class.get_id()) {
            return false;
        }
        class.assign_group(self.group_id);

        // add students to group.
        let students_inscribed = class.get_students_inscribed();
        for student_id in students_inscribed {
            if !self.students_in_group.insert(student_id) {
                // unsuscribe student if it was already on the group.
                if let Some(student) = students.get_mut(&student_id) {
                    student.unsuscribe_class(class.get_id());
                }
            }
        }
        true
    }

    pub fn matching_students(&self, class: &Class) -> u32 {
        let mut matching_students = 0;
        for student_id in class.get_students_inscribed() {
            if self.students_in_group.contains(&student_id) {
                matching_students += 1;
            }
        }
        matching_students
    }
}

pub fn assign_class_and_groups(
    mut students: HashMap<u64, Student>,
    mut classes: HashMap<u64, Class>,
) -> std::io::Result<()> {
    // Define the class groups
    let mut class_groups: HashMap<u8, ClassGroups> = HashMap::from([
        (
            1,
            ClassGroups {
                group_id: 1,
                ..Default::default()
            },
        ),
        (
            2,
            ClassGroups {
                group_id: 2,
                ..Default::default()
            },
        ),
        (
            3,
            ClassGroups {
                group_id: 3,
                ..Default::default()
            },
        ),
    ]);
    // auxiliary variable to check the classes with more inscribed people.
    let mut class_preferences: HashMap<u64, u32> = HashMap::new();
    // 1.- Give 2 first preferences to everyone.
    for (student_id, student) in students.iter_mut() {
        let class_id = student.get_class_id_by_priority(1);
        let class_id_2 = student.get_class_id_by_priority(2);
        if let Some(class) = classes.get_mut(&class_id) {
            // TODO: should this be an error?
            let _ = class.inscribe_student(student);
            let class_count = class_preferences.entry(class_id).or_insert(0);
            *class_count += 1;
        } else {
            println!("Error inscribing student with id {student_id}");
        }
        if let Some(class) = classes.get_mut(&class_id_2) {
            let _ = class.inscribe_student(student);
            let class_count = class_preferences.entry(class_id_2).or_insert(0);
            *class_count += 1;
        }
    }
    // After inscribing first 2 preferences, start assigning group to classes
    let mut classes_count_vec: Vec<_> = class_preferences.iter().collect();
    // this sort values starting from the class with less inscribed
    classes_count_vec.sort_by_key(|v| v.1);
    println!("{classes_count_vec:?}");
    // reverse to get the class with more inscribed.
    classes_count_vec.reverse();
    // assign first 3 classes to groups 1 to 3.
    if let Some(class_group) = class_groups.get_mut(&1) {
        if let Some(class) = classes.get_mut(classes_count_vec[0].0) {
            class_group.add_class_to_group(class, &mut students);
        }
        classes_count_vec.remove(0);
    }
    if let Some(class_group) = class_groups.get_mut(&2) {
        if let Some(class) = classes.get_mut(classes_count_vec[0].0) {
            class_group.add_class_to_group(class, &mut students);
        }
        classes_count_vec.remove(0);
    }
    if let Some(class_group) = class_groups.get_mut(&3) {
        if let Some(class) = classes.get_mut(classes_count_vec[0].0) {
            class_group.add_class_to_group(class, &mut students);
        }
        classes_count_vec.remove(0);
    }

    // assign the rest of the classes to other group.
    for (class_id, _) in classes_count_vec {
        // obtain class from id
        if let Some(class) = classes.get_mut(class_id) {
            let mut group_with_less_matches: u8 = 1;
            let mut less_matches = u32::MAX;
            for group in class_groups.iter_mut() {
                // TODO: Balance the classes in each group
                let matching_students = group.1.matching_students(class);
                if matching_students < less_matches {
                    less_matches = matching_students;
                    group_with_less_matches = *group.0;
                }
            }
            // inscribe to group with less matching students.
            println!(
                "class : {}, assigned to group {}",
                class.get_id(),
                group_with_less_matches
            );
            if let Some(class_group) = class_groups.get_mut(&group_with_less_matches) {
                class_group.add_class_to_group(class, &mut students);
            }
        }
    }

    let mut students_missing_classes: Vec<(u64, u8)> = vec![]; // student_id, group_id
    // once all the class are assigned to a group, inscribe students to classes
    // so each students have 3 classes from different groups.
    for (_, student) in students.iter_mut() {
        // check if student have all 3 classes.
        'group_loop: for (_, group) in class_groups.iter_mut() {
            if !group.student_in_group(student) {
                // add student to class and to group
                let pref_class_id =
                    match student.get_next_class_priority_from_group(group.classes_in_group()) {
                        Some(pref_class) => pref_class,
                        None => {
                            // add student to students that miss classes
                            // and continue with next student.
                            students_missing_classes.push((student.get_id(), group.group_id));
                            continue 'group_loop;
                        }
                    };
                if let Some(class) = classes.get_mut(&pref_class_id) {
                    let _ = group.add_student_to_class(student, class);
                }
            }
        }
    }

    // save groups to csv.
    let file = File::create("classes.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &classes)?;
    writer.flush()?;

    // write
    let students_file = File::create("students.json")?;
    let mut writer_2 = BufWriter::new(students_file);
    serde_json::to_writer(&mut writer_2, &students)?;
    writer_2.flush()?;
    Ok(())
}
