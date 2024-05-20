use std::io;
use std::io::Read;
use std::fs::File;

mod character;
mod action;

use action::ActionResult;

fn roll_for_action(player: &character::CharacterSheet) {
    println!("\nRolling for Action!");
    println!("Please select which stat to use [1-5]: ");
    println!("1) Edge");
    println!("2) Heart");
    println!("3) Iron");
    println!("4) Shadow");
    println!("5) Wits");
    let mut modifier = String::new();

    io::stdin().read_line(&mut modifier).expect("Failed to read line");

    let modifier: u8 = match modifier.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The value isn't a number, please select an option from 1 to 5.");
            return;
        },
    };

    let roll_result = match modifier {
        1 => {
            println!("\nRolling for Edge!");
            player.roll_edge()
        },
        2 => {
            println!("\nRolling for Heart!");
            player.roll_heart()
        },
        3 => {
            println!("\nRolling for Iron!");
            player.roll_iron()
        },
        4 => {
            println!("\nRolling for Shadow!");
            player.roll_shadow()
        },
        5 => {
            println!("\nRolling for Wits!");
            player.roll_wits()
        },
        _ => {
            println!("The value given doesn't correspond with any stat.");
            return;
        }
    };

    println!("Your Action Score: {}", roll_result.action);
    println!("Your Challenge Dice: {} {}", roll_result.challenge.0, roll_result.challenge.1);

    if roll_result.challenge.0 == roll_result.challenge.1 {
        println!("CRITICAL RESULT!")
    }

    match roll_result.result {
        ActionResult::StrongHit => println!("You scored a Strong Hit!!!"),
        ActionResult::WeakHit => println!("You scored a Weak Hit"),
        ActionResult::Miss => println!("Darn It, you scored a Miss!"),
    }
}

fn main() {
    let mut file = File::open("docs/example_character.json").unwrap();
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
            roll_for_action(&player_character);
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }
}
