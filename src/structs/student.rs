pub struct Student {
    // TODO: passing id as different types.
    id: u64,
    name: String,
    // TODO: Year should give priority for taking classes.
    // for the first iteration, this will not be implemented.
    year: i8,
    class_priority: Vec<u64>,
    inscribed_classes: Vec<u64>, // class id
}

impl Student {
    pub fn inscribe_class(&mut self, class_id: u64) -> () {
        self.inscribed_classes.push(class_id);
    }
}
