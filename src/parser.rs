use crate::{lexer::Lexer, token::TokenType};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) {
        self.lexer.generate_all_tokens();
        let tokens = self.lexer.tokens.clone();

        // Parse the tokens and execute the corresponding actions
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            match token.token_type {
                TokenType::Ident => {
                    let command = token.literal.to_lowercase();
                    match command.as_str() {
                        "create" => self.parse_create_table(),
                        "insert" => self.parse_insert(),
                        "select" => self.parse_select(),
                        "delete" => self.parse_delete(),
                        "save" => self.parse_save(),
                        "load" => self.parse_load(),
                        _ => println!("Unknown command: {}", command),
                    }
                }
                _ => {
                    // Handle other token types if needed
                }
            }
            i += 1;
        }
    }

    fn parse_create_table(&mut self) {
        // Parse the CREATE TABLE command and create the table
        // TODO: Implement the logic to parse the table schema and create the table
        println!("Creating table...");
    }

    fn parse_insert(&mut self) {
        // Parse the INSERT command and insert a record into the table
        // TODO: Implement the logic to parse the values and insert the record
        println!("Inserting record...");
    }

    fn parse_select(&mut self) {
        // Parse the SELECT command and query records from the table
        // TODO: Implement the logic to parse the query conditions and retrieve records
        println!("Querying records...");
    }

    fn parse_delete(&mut self) {
        // Parse the DELETE command and delete records from the table
        // TODO: Implement the logic to parse the delete conditions and remove records
        println!("Deleting records...");
    }

    fn parse_save(&mut self) {
        // Parse the SAVE command and save the database to a file
        // TODO: Implement the logic to parse the file path and save the database
        println!("Saving database...");
    }

    fn parse_load(&mut self) {
        // Parse the LOAD command and load the database from a file
        // TODO: Implement the logic to parse the file path and load the database
        println!("Loading database...");
    }
}
