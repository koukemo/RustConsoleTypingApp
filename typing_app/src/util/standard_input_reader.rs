use std::io::{self, Write};

pub struct StandardInputReader;

impl StandardInputReader {
    pub fn input_string(message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        
        input.trim().to_string()
    }

    pub fn input_number(message: &str) -> u32 {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        input.trim().parse().expect("Invalid input.")
    }
}