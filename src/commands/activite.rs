use serenity::all::ActivityData;

use super::{Context, Error};

/// Change l'activité de Staline
#[poise::command(slash_command)]
pub async fn activite(ctx: Context<'_>,     #[description = "Activité"] activite: String,
) -> Result<(), Error> {
    ctx.serenity_context().set_activity(Some(ActivityData::playing(activite)));

    ctx.reply("Activité changée.")
        .await?;

    Ok(())
}
