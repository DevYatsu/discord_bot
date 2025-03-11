use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::{ActivityData, OnlineStatus};
use shuttle_runtime::SecretStore;
use songbird::SerenityInit;
use std::sync::atomic::AtomicUsize;
use tracing::{debug, info};

mod commands;

#[derive(Debug)]
struct Data {
    activity: ActivityData,
    status: OnlineStatus,
    decret_command_count: AtomicUsize,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            activity: ActivityData::playing("Conflict of Nations"),
            status: OnlineStatus::Online,
            decret_command_count: AtomicUsize::new(0),
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    // Get the guild_id set in `Secrets.toml`
    let _guild_id = secret_store
        .get("GUILD_ID")
        .context("'GUILD_ID' was not found")?;

    let bot = Data::default();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::decret(),
                commands::goulag(),
                commands::liberation(),
                commands::hymn(),
                commands::activite()
            ],
            post_command: |ctx| {
                Box::pin(async move {
                    info!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                debug!("Setting activity and status text");
                ctx.set_presence(Some(bot.activity.clone()), bot.status);

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("rustbot logged in as {}", ready.user.name);
                Ok(bot)
            })
        })
        .build();

    let client = ClientBuilder::new(token, GatewayIntents::non_privileged())
        .framework(framework)
        .register_songbird()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
