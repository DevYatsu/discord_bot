use super::{Context, Error};
use crate::commands::goulag::GOULAG_ROLE_ID;
use poise::serenity_prelude as serenity;

/// Libère quelqu'un du goulag
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS | MOVE_MEMBERS")]
pub async fn liberation(
    ctx: Context<'_>,
    #[description = "Utilisateur sélectionné"] target_user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| {
        let _ = ctx.reply("Cette commande ne peut être utilisée que dans un serveur.");
        Error::from("Pas dans une guilde")
    })?;

    let http_ctx = ctx.http();

    if let Some(member) = guild_id.member(http_ctx, target_user.id).await.ok() {
        member.remove_role(http_ctx, GOULAG_ROLE_ID).await?;
    } else {
        ctx.reply("Utilisateur spécifié introuvable dans le serveur.")
            .await?;
        return Ok(());
    }

    Ok(())
}
