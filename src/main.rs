use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

mod character;
mod action;

fn main() {
    println!("Welcome to Rusted Iron!");
    let mut file;

    loop {
        println!("\nPlease select an option");
        println!("Create: create a new character sheet");
        println!("Load: load an existing character sheet\n");
        let mut option = String::new();
        
        io::stdin().read_line(&mut option).expect("Failed to read line");    
        let option = option.trim().to_lowercase();
        
        if option.eq(&String::from("load")) {
            let mut path_file = String::new();
            
            print!("Input the path to the .json file: ");
            std::io::stdout().flush().unwrap();
            
            io::stdin().read_line(&mut path_file).expect("Failed to read line");
            
            let trimmed_path_file = path_file.trim();
            let path = Path::new(&trimmed_path_file);
            
            if path.exists(){
                file = File::open(trimmed_path_file).unwrap();
                break;
            } else {
                println!("Path does not exist.");
                continue
            }
        // TODO: Here we need to implement the logics for creating a new character sheet.
        } else if option.eq(&String::from("create")) {
            let _ = character::character_creation();
            file = File::open("docs/example_character.json").unwrap();
            break;
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let player_character: character::CharacterSheet = serde_json::from_str(&data).expect("JSON was not well-formatted");

    println!("Loaded character sheet for {}.", player_character.name);

    loop {
        println!("\nPlease select an option ");
        println!("Action: Performs an action roll");
        println!("Exit: Exits the program\n");
        let mut option = String::new();
    
        io::stdin().read_line(&mut option).expect("Failed to read line");
        
        let option = option.trim().to_lowercase();

        if option.eq(&String::from("exit")) {
            println!("Signing out. Thanks for playing!");
            break;
        } else if option.eq(&String::from("action")) {
            action::roll_for_action(&player_character);
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }
}
