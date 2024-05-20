use std::io;
use rand::Rng;
use crate::character;

pub enum ActionResult {
    StrongHit,
    WeakHit,
    Miss
}

pub struct Action {
    pub challenge: (u8, u8),
    pub action: u8,
    pub result: ActionResult,
}

impl Action {
    pub fn roll(modifier: u8) -> Self {
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

pub fn roll_for_action(player: &character::CharacterSheet) {
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