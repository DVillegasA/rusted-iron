use std::{collections::HashMap, io};
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
    character_name.pop();

    println!("Ironsworn uses five stats, each with values from 1 to 3, that are added to your action rolls.");
    println!("  - Edge: Quickness, agility, and prowess in ranged combat.");
    println!("  - Heart: Courage, willpower, empathy, sociability, and loyalty.");
    println!("  - Iron: Physical strength, endurance, aggressiveness, and prowess in close combat.");
    println!("  - Shadow: Sneakiness, deceptiveness, and cunning.");
    println!("  - Wits: Expertise, knowledge, and observation.");
    println!("Now you will arrange the following bonuses on your character's stats: 3 2 2 1 1.\n");
    
    let mut vec_stats: Vec<String> = vec![
        "edge".to_string(), 
        "heart".to_string(), 
        "iron".to_string(), 
        "shadow".to_string(), 
        "wits".to_string()
    ];

    let mut map_stats: HashMap<String, u8> = HashMap::new();
    
    let mut bonus = 3; 
    while map_stats.len() < 3 {
        println!("Use the numbers from 1 to {} to select which stat should have a bonus of {bonus}.", vec_stats.len());
        let mut counter = 0;
        let mut stat_selection = String::new();

        for stat_ref in &vec_stats {
            counter += 1;
            print!("{counter}) {stat_ref}\n");
        }
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut stat_selection).expect("Failed to read line");

        let stat_selection: usize = match stat_selection.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => {
                eprintln!("The value isn't a number, please select an option from 1 to {}.", vec_stats.len());
                continue;
            },
        };

        let _ = match vec_stats.get(stat_selection) {
            Some(stat) => stat,
            None => {
                eprintln!("Please select an option from 1 to {}.", vec_stats.len());
                continue;
            }
        };

        let stat = vec_stats.remove(stat_selection);
        map_stats.insert(stat, bonus);

        bonus = 2;
    }

    for stat in vec_stats {
        map_stats.insert(stat, 1);
    }

    let stats = Stats {
        edge: *map_stats.get("edge").unwrap(),
        heart: *map_stats.get("heart").unwrap(),
        iron: *map_stats.get("iron").unwrap(),
        shadow: *map_stats.get("shadow").unwrap(),
        wits: *map_stats.get("wits").unwrap(),
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
