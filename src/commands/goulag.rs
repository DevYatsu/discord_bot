use super::{Context, Error};
use ::serenity::{
    all::{EditChannel, RoleId},
    http,
};
use poise::serenity_prelude as serenity;
use serenity::all::{
    ChannelType, CreateChannel, PermissionOverwrite, PermissionOverwriteType, Permissions,
};

pub const GOULAG_ROLE_ID: RoleId = RoleId::new(1345767210236248165);

/// Envoie quelqu'un au goulag
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS | MOVE_MEMBERS")]
pub async fn goulag(
    ctx: Context<'_>,
    #[description = "Utilisateur sélectionné"] target_user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| {
        let _ = ctx.reply("Cette commande ne peut être utilisée que dans un serveur.");
        Error::from("Pas dans une guilde")
    })?;

    let http_ctx = ctx.http();

    let prison_channel_name = 

    if  ctx.author().id == 1253028327418101832 {
"fleury"
    }else {"goulag"};

    let builder = CreateChannel::new(prison_channel_name)
        .kind(ChannelType::Voice)
        .permissions(vec![
            // Deny @everyone all access (including visibility)
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
                kind: PermissionOverwriteType::Role(serenity::all::RoleId::new(guild_id.into())),
            },
            // Allow target user to view but not leave
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
                deny: Permissions::MANAGE_CHANNELS, // Prevents moving themselves
                kind: PermissionOverwriteType::Member(target_user.id),
            },
            // Allow command issuer full control
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL
                    | Permissions::CONNECT
                    | Permissions::MANAGE_CHANNELS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(ctx.author().id),
            },
        ]);

    let existing_channels = guild_id.channels(http_ctx).await?;

    let mut goulag_channel = match existing_channels
        .iter()
        .find(|(_, channel)| channel.name == prison_channel_name)
    {
        Some((_, channel)) => channel.clone(),
        None => guild_id.create_channel(http_ctx, builder).await?,
    };

    if let Some(member) = guild_id.member(http_ctx, target_user.id).await.ok() {
        match member
            .move_to_voice_channel(http_ctx, goulag_channel.id)
            .await
        {
            Ok(_) => {
                ctx.reply(format!("{} a été envoyé au camp !", target_user.name))
                    .await?;

                //giving the role GOULAG to the user
                member.add_role(http_ctx, GOULAG_ROLE_ID).await?;

                let builder = EditChannel::new().permissions(vec![PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::CONNECT,
                    kind: PermissionOverwriteType::Member(target_user.id),
                }]);

                goulag_channel.edit(http_ctx, builder).await?;
            }
            Err(why) => {
                if let serenity::Error::Http(http_err) = &why {
                    match http_err {
                        http::HttpError::UnsuccessfulRequest(error_response) => {
                            if error_response.error.message
                                == "Target user is not connected to voice."
                            {
                                ctx.reply(format!(
                                    "{} n'est pas connecté à un salon vocal.",
                                    target_user.name
                                ))
                                .await?;
                            } else {
                                eprintln!("Erreur HTTP : {:?}", error_response.error.message);
                                ctx.reply("Échec de l'envoi au Goulag.").await?;
                            }
                        }
                        e => println!("HTTP error: {:?}", e),
                    };
                }
                eprintln!("Erreur lors du déplacement de l'utilisateur : {:?}", why);
                let _ = goulag_channel.delete(http_ctx).await;
                return Ok(());
            }
        }
    } else {
        ctx.reply("Utilisateur spécifié introuvable dans le serveur.")
            .await?;
        let _ = goulag_channel.delete(http_ctx).await;
        return Ok(());
    }

    Ok(())
}
