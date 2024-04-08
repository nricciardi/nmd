use std::ops::Add;

use super::Modifier;

#[derive(Debug, PartialEq, Clone)]
pub enum ModifiersBucket {
    All,
    List(Vec<Box<dyn Modifier>>),
    None
}

impl ModifiersBucket {
    pub fn contains(&self, searched_modifier: &Box<dyn Modifier>) -> bool {
        match self {
            Self::All => true,
            Self::List(modifiers_list) => modifiers_list.contains(searched_modifier),
            Self::None => false,
        }
    }
}

impl Add for ModifiersBucket {
    type Output = Self;

    fn add(self, new_modifiers_excluded: Self) -> Self::Output {
        match new_modifiers_excluded.clone() {
            Self::All => Self::All,
            Self::List(mut modifiers_to_add) => {
                match self {
                    Self::All => return Self::All,
                    Self::List(mut modifiers_already_excluded) => {
                        modifiers_already_excluded.append(&mut modifiers_to_add);

                        return Self::List(modifiers_already_excluded)
                    },
                    Self::None => return new_modifiers_excluded.clone(),
                }
            },
            Self::None => return self
        }
    }
}