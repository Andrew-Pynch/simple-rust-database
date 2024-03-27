use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

struct Database {
    data: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Database {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());

        // log the insert operation and the current size of the db;
        println!(
            "Insert operation: key: {}, value: {}, size: {}",
            key,
            value,
            self.data.len()
        );
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    fn load(&mut self, file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut data = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, '\t').collect();
            if parts.len() == 2 {
                data.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        Ok(Database { data })
    }

    fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?;
        for (key, value) in &self.data {
            writeln!(file, "{}\t{}", key, value)?;
        }
        Ok(())
    }
}
