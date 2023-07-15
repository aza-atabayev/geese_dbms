use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Deserialize)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Deserialize)]
pub struct Database {
    pub tables: Vec<Table>,
}

pub fn get_tables(path: &str) -> Option<Database> {
    let metadata = fs::metadata(path);
    if metadata.unwrap().len() == 0 
    {
        return None;
    }
    let file = File::open(&Path::new(path));
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let db: Result<Database, _> = serde_json::from_reader(reader);
            match db {
                Ok(db) => Some(db),
                Err(e) => {
                    println!("Error parsing JSON: {}", e);
                    None
                }
            }
        }
        Err(_) => {
            println!("Unable to open file");
            None
        }
    }
}

