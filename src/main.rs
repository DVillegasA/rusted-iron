use std::{fs, io};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

mod character;
mod action;

fn main() {
    println!("Welcome to Rusted Iron!");
    let player_character: character::CharacterSheet;
    
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
                let mut data = String::new();
                let mut file = File::open(trimmed_path_file).unwrap();
                file.read_to_string(&mut data).unwrap();
                player_character = serde_json::from_str(&data).expect("JSON was not well-formatted");
                break;
            } else {
                println!("Path does not exist.");
                continue
            }
        } else if option.eq(&String::from("create")) {
            player_character = character::character_creation();

            let mut path_name = player_character.name.clone().replace(' ', "_");
            path_name.push_str(".json");
            let path = Path::new(&path_name);
            let data = serde_json::to_string(&player_character).unwrap();
            fs::write(path, data).expect("Unable to write file.");
            break;
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }

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
