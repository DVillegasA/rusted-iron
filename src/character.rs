use serde::{Deserialize, Serialize};
use crate::Action;

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