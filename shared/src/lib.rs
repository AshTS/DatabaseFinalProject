use serde::{Serialize, Deserialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Spell {
    pub name: String,
    pub range: usize, // Feet
    pub duration: usize, // In Seconds
    pub level: u8
}


/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpellRequirement {
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Class {
    Bard,
    BlackGuard,
    Cleric,
    Druid,
    Paladin,
    Ranger,
    Wizard
}

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpellClass {
    class: Class,
}

pub fn get_sample_spells() -> Vec<Spell> {
    vec![
        Spell { name: "Flarbgarber".into(), range: 42, duration: 3600, level: 5 },
        Spell { name: "Gwiflepoof".into(), range: 7, duration: 120, level: 4 },
        Spell { name: "Kartknocker".into(), range: 100, duration: 300, level: 2 }
    ]
}