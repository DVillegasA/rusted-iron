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

fn roll_for_action(player: &character::CharacterSheet) {
    println!("\nPlease select which stat to use: ");
    println!("1) Edge");
    println!("2) Heart");
    println!("3) Iron");
    println!("4) Shadow");
    println!("5) Wits");
    let mut modifier = String::new();

    io::stdin().read_line(&mut modifier).expect("Failed to read line");

    let modifier: u8 = match modifier.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let roll_result = match modifier {
        1 => player.roll_edge(),
        2 => player.roll_heart(),
        3 => player.roll_iron(),
        4 => player.roll_shadow(),
        5 => player.roll_wits(),
        _ => {
            println!("The value given doesn't correspond with any stat.");
            return;
        }
    };

    println!("\nYour Action Score: {}", roll_result.action);
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

fn main() {
    let mut file = File::open("docs/example_character.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let player_character: character::CharacterSheet = serde_json::from_str(&data).expect("JSON was not well-formatted");

    println!("Loaded character sheet for {}.", player_character.name);
    println!("Rolling for Action!");

    loop {
        println!("\nPlease select an option ");
        println!("Action: Performs an action roll");
        println!("Exit: Exits the program");
        let mut option = String::new();
    
        io::stdin().read_line(&mut option).expect("Failed to read line");
        
        let option = option.trim().to_lowercase();
        // End the program if the user inputs "exit".
        if option.eq(&String::from("exit")) {
            break;
        } else if option.eq(&String::from("action")) {
            roll_for_action(&player_character);
        } else {
            println!("Please input a valid option.");
            continue;
        }
    }
}
