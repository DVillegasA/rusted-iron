use rand::Rng;
use std::io;
use std::io::Read;
use std::fs::File;

mod character;

enum ActionResult {
    StrongHit,
    WeakHit,
    Miss
}

struct Action {
    challenge: (u8, u8),
    action: u8,
    result: ActionResult,
}

impl Action {
    fn roll(modifier: u8) -> Self {
        let mut gen = rand::thread_rng();
        let challenge_1 = gen.gen_range(1..=10);
        let challenge_2 = gen.gen_range(1..=10);
        let action = gen.gen_range(1..=6) + modifier;
        let result: ActionResult;

        if challenge_1 >= action && challenge_2 >= action {
            result = ActionResult::Miss;
        } else if challenge_1 < action && challenge_2 < action {
            result = ActionResult::StrongHit;
        } else {
            result = ActionResult::WeakHit;
        }

        Action { 
            challenge: (challenge_1, challenge_2), 
            action: action, 
            result: result 
        }
    }

}


fn main() {
    let mut file = File::open("docs/example_character.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let player_character: character::CharacterSheet = serde_json::from_str(&data).expect("JSON was not well-formatted");

    println!("Loaded character sheet for {}.", player_character.name);
    println!("Rolling for Action!");

    loop {
        println!("\nPlease select which stat to use: ");
        println!("1) Edge");
        println!("2) Heart");
        println!("3) Iron");
        println!("4) Shadow");
        println!("5) Wits");
        let mut modifier = String::new();
    
        io::stdin().read_line(&mut modifier).expect("Failed to read line");
        
        // End the program if the user inputs "exit".
        if modifier.trim().to_lowercase().eq(&String::from("exit")) {
            break;
        }

        let modifier: u8 = match modifier.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        let roll_result = match modifier {
            1 => player_character.roll_edge(),
            2 => player_character.roll_heart(),
            3 => player_character.roll_iron(),
            4 => player_character.roll_shadow(),
            5 => player_character.roll_wits(),
            _ => {
                println!("The value given doesn't correspond with any stat.");
                continue;
            }
        };
    
        println!("Your Action Score: {}", roll_result.action);
        println!("Your Challenge Dice: {} {}", roll_result.challenge.0, roll_result.challenge.1);

        if roll_result.challenge.0 == roll_result.challenge.1 {
            println!("CRITICAL RESULT!")
        }

        match roll_result.result {
            ActionResult::StrongHit => println!("You score a Strong Hit!!!"),
            ActionResult::WeakHit => println!("You score a Weak Hit"),
            ActionResult::Miss => println!("Darn It, you score a Miss!"),
        }
    }
}
