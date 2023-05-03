use serde::{Serialize, Deserialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Spell {
    pub name: String,
    pub range: usize, // Feet
    pub duration: usize, // In Seconds
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub spell_id: usize,
    pub area: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct SpellPair {
    #[serde(flatten)]
    pub spell: Spell,
    pub pairs: Vec<ClassLevelPair>
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ClassLevelPair {
    pub spell_id: usize,
    pub class: Class,
    pub level: u8
}

impl std::default::Default for Class {
    fn default() -> Self {
        Class::Bard
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Bard => write!(f, "Bard"),
            Class::BlackGuard => write!(f, "BlackGuard"),
            Class::Cleric => write!(f, "Cleric"),
            Class::Druid => write!(f, "Druid"),
            Class::Paladin => write!(f, "Paladin"),
            Class::Ranger => write!(f, "Ranger"),
            Class::Wizard => write!(f, "Wizard"),
        }
    }
}

impl Class {
    pub fn color(&self) -> &'static str {
        match self {
            Class::Bard => "primary",
            Class::BlackGuard => "dark",
            Class::Cleric => "info",
            Class::Druid => "success",
            Class::Paladin => "warning",
            Class::Ranger => "danger",
            Class::Wizard => "link",
        }
    }
}