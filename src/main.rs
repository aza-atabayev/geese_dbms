use std::io::{self, Write};
use std::process;
mod cli;

mod utils{
    pub mod cmd_io;
    pub mod directory;
}

fn intro() {
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
            intro(); 
        }
        _ => {
            let choice = input.parse::<usize>();
            match choice {
                Ok(database_num) => {
                    if database_num <= db_names.len() && database_num > 0 {
                        let selected_database = &db_names[database_num - 1];
                        // Code for accessing the selected database goes here
                        println!("Accessing database '{}'", selected_database);
                    } else {
                        println!("Invalid database number.");
                    }
                }
                Err(_) => {
                    println!("Invalid input.");
                    intro();
                }
            }
        }
    }
}
    

fn main() {
    intro();
    
    // 3. current_db = something and show that they can use "disconnect" to disconnect

    // 4. read db from the disk and pass it is a reference 

    // 5. go into query processing mode until "disconnect"

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