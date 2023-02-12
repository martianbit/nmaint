use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;

pub enum MenuItemKind<'a> {
    Basic(&'a [MenuItem<'a>]),
    Checkbox(bool),
}

#[derive(Copy, Clone)]
pub enum MenuItemMeaning {
    Update,
    Upgrade,
    Use,
}

impl Display for MenuItemMeaning {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Update => "update pkg db",
            Self::Upgrade => "upgrade system",
            Self::Use => "config use flags",
        })
    }
}

pub struct MenuItem<'a> {
    pub kind: MenuItemKind<'a>,
    pub meaning: MenuItemMeaning,
}

