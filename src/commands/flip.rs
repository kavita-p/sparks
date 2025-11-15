use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::random_bool;

#[poise::command(
    slash_command,
    description_localized("en-US", "Flips a coin.")
)]
pub async fn flip(
    ctx: Context<'_>
) -> Result<(), Error> {

    let face = if random_bool(0.5) { "heads" } else { "tails" };

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("Clink!")
                .description(format!("Got **{face}.**"))
                .color(serenity::Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
