use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot = Robot {
            name: String::new(),
        };
        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_unique_name();
    }

    fn generate_unique_name() -> String {
        let mut rng = rand::thread_rng();
        let mut used_names = USED_NAMES.lock().unwrap();

        loop {
            // Generate two random uppercase letters
            let letter1 = (b'A' + rng.gen_range(0..26)) as char;
            let letter2 = (b'A' + rng.gen_range(0..26)) as char;

            // Generate three random digits
            let digit1 = rng.gen_range(0..10).to_string();
            let digit2 = rng.gen_range(0..10).to_string();
            let digit3 = rng.gen_range(0..10).to_string();

            // Combine to create name
            let name = format!("{}{}{}{}{}", letter1, letter2, digit1, digit2, digit3);

            // Check if name is unique
            if !used_names.contains(&name) {
                used_names.insert(name.clone());
                return name;
            }
        }
    }
}
