use std::fs;

use crate::utils::cmd_io::read_input;
use crate::utils::json::{Database, Table, Field};

pub fn print_intro() {
    println!();
    println!("𓅬 Welcome to Geese (GIS) DBMS 𓅬");
    println!("----------------------------------------------");
    println!();
}

pub fn print_choice(is_empty: bool) {
    println!("What would you like to do?");
    println!("- To create a new database, enter 'create'");
    if !is_empty {println!("- To access an existing database, enter the corresponding number")};
    println!("- To exit, enter 'exit'");
    println!();
}

pub fn handle_db_creation() {
    println!("Enter database name: ");
    let db_name = read_input();
    if let Err(err) = fs::create_dir(format!("./data/{}.db", db_name)) {
        eprintln!("Error creating database: {}", err);
    }
}

pub fn print_tables(db: Option<Database>) {
    match db {
        Some(db) => {
            if db.tables.is_empty() {
                println!("No tables are found in the database.");
            } else {
                for table in db.tables {
                    println!("Table: {}", table.name);
                    for field in table.fields {
                        println!("    Field: {}, Type: {}", field.name, field.field_type);                       
                    }
                    println!();
                }
            }
        }
        None => println!("_structure.json is empty, no tables to fetch."),
    }
}