use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::{rng, distr::{Bernoulli, Distribution}, random_bool};

#[poise::command(
    slash_command,
    description_localized("en-US", "Flips a coin.")
)]
pub async fn flip(
    ctx: Context<'_>,
    #[description = "The label for the heads face."]
    heads: Option<String>,
    #[description = "The label for the tails face."]
    tails: Option<String>
) -> Result<(), Error> {

    let heads = heads.unwrap_or("heads".to_string());
    let tails = tails.unwrap_or("tails".to_string());

    let face = if random_bool(0.5) { heads } else { tails };

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

#[poise::command(
    slash_command,
    description_localized("en-US", "Flips multiple coins.")
)]
pub async fn manyflip(
    ctx: Context<'_>,
    #[description = "The number of coins to flip."]
    #[min = 1]
    count: Option<i64>,
    #[description = "The label for the heads face."]
    heads: Option<String>,
    #[description = "The label for the tails face."]
    tails: Option<String>
) -> Result<(), Error> {
    let count = count.unwrap_or(1).try_into().unwrap_or(1);

    let heads = heads.unwrap_or("heads".to_string());
    let tails = tails.unwrap_or("tails".to_string());

    let faces = Bernoulli::new(0.5)
        .unwrap()
        .sample_iter(rng())
        .take(count)
        .map(|coin| {
            if coin {
                heads.clone()
            } else {
                tails.clone()
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title("Clink!")
                .description(format!("Got **{faces}.**"))
                .color(serenity::Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
