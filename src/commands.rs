mod quotes;
use crate::{Context, Error, commands::quotes::generate_quote};
use tokio::task;

/// Responds with "my friend!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    let fetched_quote = task::spawn_blocking( move || {
        generate_quote()
    }).await?.unwrap();
    let message = format!("Hello, my friend!\n\nI was just reading about famous quotes today. Here's one that I wanted to share:\n\n{fetched_quote}");
    ctx.say(message).await?;
    Ok(())
}
/// Responds with "pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}
