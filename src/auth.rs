use std::io::{self, Write};

pub struct Auth;

impl Auth {
    pub fn authenticate() -> Result<(), String> {
        println!("Please login to continue");

        let username = Self::read_input("Username:")?;
        let password = Self::read_input("Password:")?;

        if username == "store_manager" && password == "pass1234" {
            Ok(())
        } else {
            Err("Invalid username or password".to_string())
        }
    }

    fn read_input(prompt: &str) -> Result<String, String> {
        print!("{prompt} ");
        io::stdout()
            .flush()
            .map_err(|_| "Failed to flush stdout. Please try again")?;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|err| err.to_string())?;
        Ok(input.trim().to_string())
    }
}
