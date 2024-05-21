use std::io;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    edge: u8,
    heart: u8,
    iron: u8,
    shadow: u8,
    wits: u8
}

#[derive(Serialize, Deserialize)]
pub struct Momentum {
    current: i8,
    max: u8,
    reset: u8
}

#[derive(Serialize, Deserialize)]
struct Conditions {
    wounded: bool,
    shaken: bool,
    unprepared: bool,
    encumbered: bool
}

#[derive(Serialize, Deserialize)]
struct Banes {
    maimed: bool,
    corrupted: bool
}

#[derive(Serialize, Deserialize)]
struct Burdens {
    cursed: bool,
    tormented: bool
}

#[derive(Serialize, Deserialize)]
pub struct Debilities {
    conditions: Conditions,
    banes: Banes,
    burdens: Burdens
}

#[derive(Serialize, Deserialize)]
pub struct CharacterSheet {
    pub name: String,
    pub experience: u8,
    pub stats: Stats,
    pub health: u8,
    pub spirit: u8,
    pub supply: u8,
    pub momentum: Momentum,
    pub debilities: Debilities
}

impl CharacterSheet {
    pub fn roll_edge(&self) -> Action {
        Action::roll(self.stats.edge)
    }
    pub fn roll_heart(&self) -> Action {
        Action::roll(self.stats.heart)
    }
    pub fn roll_iron(&self) -> Action {
        Action::roll(self.stats.iron)
    }
    pub fn roll_shadow(&self) -> Action {
        Action::roll(self.stats.shadow)
    }
    pub fn roll_wits(&self) -> Action {
        Action::roll(self.stats.wits)
    }
}

pub fn character_creation() -> CharacterSheet {
    println!("Creating a new character.\n");
    
    let mut character_name = String::new();
    print!("\nProvide the name for your new character: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut character_name).expect("Failed to read line");

    let mut vec_stats: Vec<String> = vec![
        "edge".to_string(), 
        "heart".to_string(), 
        "iron".to_string(), 
        "shadow".to_string(), 
        "wits".to_string()
    ];

    println!("Ironsworn uses five stats, each with values from 1 to 3, that are added to your action rolls.");
    println!("  - Edge: Quickness, agility, and prowess in ranged combat.");
    println!("  - Heart: Courage, willpower, empathy, sociability, and loyalty.");
    println!("  - Iron: Physical strength, endurance, aggressiveness, and prowess in close combat.");
    println!("  - Shadow: Sneakiness, deceptiveness, and cunning.");
    println!("  - Wits: Expertise, knowledge, and observation.");
    println!("Now you will arrange the following bonuses on your character's stats: 3 2 2 1 1");

    let stats = Stats {
        edge: 0,
        heart: 0,
        iron: 0,
        shadow: 0,
        wits: 0
    };

    CharacterSheet {
        name: character_name,
        experience: 0,
        stats: stats,
        health: 5,
        spirit: 5,
        supply: 5,
        momentum: Momentum {
            current: 2,
            max: 10,
            reset: 2
        },
        debilities: Debilities {
            conditions: Conditions {
                wounded: false,
                shaken: false,
                unprepared: false,
                encumbered: false
            },
            banes: Banes {
                maimed: false,
                corrupted: false
            },
            burdens: Burdens {
                cursed: false,
                tormented: false
            }
        }
    }
}
