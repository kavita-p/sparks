use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::{seq::SliceRandom, Rng};
use serenity::all::Color;

const EMBED_GREEN: Color = Color::from_rgb(87, 242, 135);

const STANDARD_SUITS: [&str; 4] = ["Hearts", "Spades", "Clubs", "Diamonds"];
const STANDARD_RANKS: [&str; 13] = [
    "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen",
    "King",
];

const TAROT_SUITS: [&str; 4] = ["Wands", "Cups", "Swords", "Pentacles"];
const TAROT_RANKS: [&str; 14] = [
    "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Page",
    "Knight", "Queen", "King",
];
const MAJOR_ARCANA: [&str; 22] = [
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

#[derive(poise::ChoiceParameter)]
enum Deck {
    Standard,
    Tarot,
}

fn draw_tarot() -> (String, &'static str) {
    let direction = if rand::random::<bool>() {
        "Upright"
    } else {
        "Reversed"
    };

    let card_type = if rand::thread_rng().gen_range(1..79) > 22 {
        let (suit, rank) = (
            TAROT_SUITS.choose(&mut rand::thread_rng()).unwrap(),
            TAROT_RANKS.choose(&mut rand::thread_rng()).unwrap(),
        );
        format!("{rank} of {suit}, {direction}")
    } else {
        format!(
            "{}, {}",
            MAJOR_ARCANA.choose(&mut rand::thread_rng()).unwrap(),
            direction
        )
    };

    (card_type, "tarot deck")
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Generates a random card without replacement.")
)]
pub async fn draw(ctx: Context<'_>, deck: Option<Deck>) -> Result<(), Error> {
    let (title, deck_desc) = match deck {
        Some(Deck::Tarot) => draw_tarot(),
        _ => (
            format!(
                "{} of {}",
                STANDARD_RANKS.choose(&mut rand::thread_rng()).unwrap(),
                STANDARD_SUITS.choose(&mut rand::thread_rng()).unwrap()
            ),
            "standard deck of cards",
        ),
    };

    let response = poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(title)
            .description(format!("From a {deck_desc}."))
            .color(EMBED_GREEN),
    );
    ctx.send(response).await?;
    Ok(())
}
