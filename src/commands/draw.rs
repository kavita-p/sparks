use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use serenity::all::Color;

const EMBED_GREEN: Color = Color::from_rgb(87, 242, 135);
const SUITS: [&str; 4] = ["Hearts", "Spades", "Clubs", "Diamonds"];
const RANKS: [&str; 13] = [
    "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen",
    "King",
];

#[poise::command(
    slash_command,
    description_localized("en-US", "Generates a random card without replacement.")
)]
pub async fn draw(ctx: Context<'_>) -> Result<(), Error> {
    let suit = SUITS.choose(&mut rand::thread_rng()).unwrap();
    let rank = RANKS.choose(&mut rand::thread_rng()).unwrap();

    let response = poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("{rank} of {suit}"))
            .description("From a standard deck of cards.")
            .color(EMBED_GREEN),
    );
    ctx.send(response).await?;
    Ok(())
}
