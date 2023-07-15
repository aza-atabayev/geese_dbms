use std::process;
mod cli;

mod utils;
use crate::utils::json::get_tables;



fn intro() -> String {
    cli::print_intro();

    let db_names = utils::directory::get_file_names("./data/");
    let is_empty = db_names.is_empty();

    if !is_empty {
        println!("Available databases:");

        for (index, db_name) in db_names.iter().enumerate() {
            println!("{}) {}", index + 1, db_name);
        }
        println!("")
    }

    cli::print_choice(is_empty);

    let input = utils::cmd_io::read_input();

    match input.as_str() {
        "exit" => {
            println!("Exiting...");
            process::exit(0); 
        }   
        "create" => {
            cli::handle_db_creation();
            intro()
        }
        _ => {
            let choice = input.parse::<usize>();
            match choice {
                Ok(database_num) => {
                    if database_num <= db_names.len() && database_num > 0 {
                        let selected_database = &db_names[database_num - 1];
                        // Code for accessing the selected database goes here
                        println!("Accessing database '{}'", selected_database);
                        selected_database.to_string()
                    } else {
                        println!("Invalid database number.");
                        intro()
                    }
                }
                Err(_) => {
                    println!("Invalid input.");
                    intro()
                }
            }
        }
    }
}
    

fn main() {
    let current_db = intro();
    let tables = get_tables(&format!("./data/{}/_structure.json", current_db));
    // TODO: read all geoJson files and convert them into 1 big r-tree

    cli::print_tables(tables);
    // 4. read db from the disk and pass it is a reference 

    // 5. go into query processing mode until exit
    cli::handle_queries();

}
