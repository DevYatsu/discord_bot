use super::{Context, Error};
use songbird::{get, input::YoutubeDl};

/// Joue l'hymne du peuple
#[poise::command(slash_command, track_edits)]
pub async fn hymn(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| {
        let _ = ctx.reply("Cette commande ne peut être utilisée que dans un serveur.");
        Error::from("Pas dans une guilde")
    })?;

    let manager = get(ctx.serenity_context())
        .await
        .ok_or("Voice manager not found.")?
        .clone();

    let channel_id = guild_id
        .to_guild_cached(&ctx.cache())
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|state| state.channel_id);

    if let None = channel_id {
        ctx.reply("Vous devez être dans un salon vocal pour utiliser cette commande.")
            .await?;
        return Ok(());
    }

    let call = manager.join(guild_id, channel_id.unwrap()).await?;

    let audio_url = "https://www.youtube.com/watch?v=U06jlgpMtQs&ab_channel=rascrifice"; // urss anthem
    let source = YoutubeDl::new(reqwest::Client::new(), audio_url);

    // Play the audio
    let mut handler = call.lock().await;
    handler.play_input(source.clone().into());


    ctx.reply("Lecture de l'hymne nationale")
        .await?;

    Ok(())
}
