pub mod text_modifier;
pub mod paragraph_modifier;
pub mod chapter_modifier;

use std::{fmt::{Debug, Display}, ops::Add};


pub const MAX_HEADING_LEVEL: u32 = 6; 

pub trait Modifier {
    fn search_pattern(&self) -> String;

    fn incompatible_modifiers(&self) -> Modifiers {
        Modifiers::None
    }
}

impl Display for dyn Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.search_pattern())
    }
}

impl Debug for dyn Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Modifier:\nsearch_pattern='{}'\nincompatible_modifiers={:?}", self.search_pattern(), self.incompatible_modifiers())
    }
}

impl PartialEq for dyn Modifier {
    fn eq(&self, other: &Self) -> bool {
        self.search_pattern().eq(&other.search_pattern())
    }
}

impl Clone for Box<dyn Modifier> {
    fn clone(&self) -> Self {
        self.clone()
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Modifiers {
    All,
    List(Vec<Box<dyn Modifier>>),
    None
}

impl Modifiers {
    pub fn contains(&self, searched_modifier: &Box<dyn Modifier>) -> bool {
        match self {
            Modifiers::All => true,
            Modifiers::List(modifiers_list) => modifiers_list.contains(searched_modifier),
            Modifiers::None => false,
        }
    }
}

impl Add for Modifiers {
    type Output = Modifiers;

    fn add(self, new_modifiers_excluded: Self) -> Self::Output {
        
        match new_modifiers_excluded {
            Modifiers::All => Self::All,
            Modifiers::List(mut modifiers_to_add) => {
                match self {
                    Modifiers::All => return Self::All,
                    Modifiers::List(mut modifiers_already_excluded) => {
                        modifiers_already_excluded.append(&mut modifiers_to_add);

                        return Modifiers::List(modifiers_already_excluded)
                    },
                    Modifiers::None => return new_modifiers_excluded,
                }
            },
            Modifiers::None => return self
        }
    }
}
