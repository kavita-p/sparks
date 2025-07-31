use core::fmt;
use std::fmt::Display;

#[derive(poise::ChoiceParameter)]
enum Deck {
    Standard,
    Tarot,
}

impl Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Tarot => write!(f, "tarot"),
        }
    }
}

pub mod draw;
pub mod misc;
pub mod roll;
