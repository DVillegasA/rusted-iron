use rand::Rng;
use std::io;

enum ActionResult {
    StrongHit,
    WeakHit,
    Miss
}

struct Action {
    challenge: (u32, u32),
    action: u32,
    result: ActionResult,
}

impl Action {
    fn roll(modifier: u32) -> Self {
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
    println!("Rolling for Action!");

    loop {
        println!("\nPlease input your modifier: ");
        let mut modifier = String::new();
    
        io::stdin().read_line(&mut modifier).expect("Failed to read line");
        
        // End the program if the user inputs "exit".
        if modifier.trim().to_lowercase().eq(&String::from("exit")) {
            break;
        }

        let modifier: u32 = match modifier.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        let roll_result = Action::roll(modifier);
    
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
