use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult{
    msg.channel_id.say(&ctx.http, "world!").await?;
    Ok(())
}