use crate::{Context, Error};
use core::fmt;
use itertools::iproduct;
use lazy_static::lazy_static;
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use serenity::all::Color;
use std::fmt::Display;

const EMBED_GREEN: Color = Color::from_rgb(87, 242, 135);

lazy_static! {
    static ref STANDARD_SUITS: Vec<&'static str> = vec!["Hearts", "Spades", "Clubs", "Diamonds"];
    static ref STANDARD_RANKS: Vec<&'static str> = vec![
        "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
        "Queen", "King",
    ];
    static ref TAROT_SUITS: Vec<&'static str> = vec!["Wands", "Cups", "Swords", "Pentacles"];
    static ref TAROT_RANKS: Vec<&'static str> = vec![
        "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
        "Queen",
    ];
    static ref MAJOR_ARCANA: Vec<&'static str> = vec![
        "0 - The Fool",
        "I - The Magician",
        "II - The High Priestess",
        "III - The Empress",
        "IV - The Emperor",
        "V - The Hierophant",
        "VI - The Lovers",
        "VII - The Chariot",
        "VIII - Strength",
        "IX - The Hermit",
        "X - Wheel of Fortune",
        "XI - Justice",
        "XII - The Hanged Man",
        "XIII - Death",
        "XIV - Temperance",
        "XV - The Devil",
        "XVI - The Tower",
        "XVII - The Star",
        "XVIII - The Moon",
        "XIX - The Sun",
        "XX - Judgement",
        "XXI - The World",
    ];
}

#[derive(PartialEq, poise::ChoiceParameter)]
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

fn gen_hand(deck_type: &Deck, hand_size: usize) -> Vec<String> {
    let deck_iter = if *deck_type == Deck::Tarot {
        iproduct!(TAROT_RANKS.iter(), TAROT_SUITS.iter())
    } else {
        iproduct!(STANDARD_RANKS.iter(), STANDARD_SUITS.iter())
    };

    let mut deck: Vec<_> = deck_iter
        .map(|(rank, suit)| {
            let mut card = format!("{rank} of {suit}");
            if matches!(deck_type, Deck::Tarot) {
                let direction = if rand::random::<bool>() {
                    "Upright"
                } else {
                    "Reversed"
                };
                card.push_str(&format!(", {direction}"));
            };
            card
        })
        .collect();

    if matches!(deck_type, Deck::Tarot) {
        deck.extend(MAJOR_ARCANA.iter().map(|card| {
            let direction = if rand::random::<bool>() {
                "Upright"
            } else {
                "Reversed"
            };
            format!("{card}, {direction}")
        }));
    };

    let hand = deck
        .choose_multiple(&mut rand::thread_rng(), hand_size)
        .collect::<Vec<_>>();

    hand.iter().map(std::string::ToString::to_string).collect()
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Generates a random hand_size of cards.")
)]
pub async fn draw(ctx: Context<'_>, deck: Option<Deck>, _hand: Option<i32>) -> Result<(), Error> {
    let deck = deck.unwrap_or(Deck::Standard);
    let hand = gen_hand(&deck, 5);

    let response = poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .description(hand.join("\n"))
            .fields(vec![("Deck".to_string(), format!("{deck}"), true)])
            .color(EMBED_GREEN),
    );
    ctx.send(response).await?;
    Ok(())
}
