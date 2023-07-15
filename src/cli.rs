use std::fs;
use std::io::{self, Write};
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
    let db_folder = format!("./data/{}.db", db_name);
    if let Err(err) = fs::create_dir(&db_folder) {
        eprintln!("Error creating database folder: {}", err);
        return;
    }

    let structure_file_path = format!("{}/_structure.json", db_folder);
    if let Err(err) = fs::File::create(&structure_file_path) {
        eprintln!("Error creating structure file: {}", err);
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

pub fn handle_queries() {
    loop {
        let mut input = String::new(); // create a mutable string to store our input


        print!("Please enter a query (or type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // print prompt and flush it to stdout
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // read a line from stdin
    
        let input = input.trim(); // remove trailing newline

        if input == "exit" {
            break;
        }

        println!("Processing query: {}", input);

        // Add the logic to process the input query here
    }
}