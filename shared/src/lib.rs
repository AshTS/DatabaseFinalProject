use serde::{Serialize, Deserialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Spell {
    pub name: String,
    pub range: usize, // Feet
    pub duration: usize, // In Seconds
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool
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

impl std::default::Default for Class {
    fn default() -> Self {
        Class::Bard
    }
}

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct SpellClass {
    class: Class,
}