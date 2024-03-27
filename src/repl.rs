use std::io::Write;

use crate::parser::Parser;

pub const PROMPT: &str = "pdb >> ";

pub fn start() {
    println!("Welcome to the pynch database program");

    loop {
        // Print the prompt
        print!("pdb >> ");
        std::io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Remove trailing newline character
        input = input.trim().to_string();

        // Check if the user wants to exit
        if input == "exit" {
            println!("Exiting the program.");
            break;
        }

        let mut parser = Parser::new(&input);
        parser.parse();
    }
}
