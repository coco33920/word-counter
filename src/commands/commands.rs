use poise::{samples::HelpConfiguration, serenity_prelude::{Error, GetMessages, Message, User}, Context};

use crate::{utils::utils, Data};

#[poise::command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_, Data, Error>,
    #[description = "the command"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        HelpConfiguration {
            extra_text_at_bottom: "Programmed with loved by Charlotte Thomas.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command,slash_command)]
pub async fn count_user_words(
    ctx: Context<'_, Data, Error>,
    #[description = "selected user"] user: Option<User>
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let channels = ctx.guild().unwrap().channels(ctx.http()).await?;
    let ignored_channels = vec!["".to_string()];
    let mut total_words = 0;
    let mut total_messages = Vec::new();
    for (id,channel) in channels {
        if ignored_channels.contains(&id.to_string()) {
            continue;
        }
    
        let message = channel.messages(ctx.http(), GetMessages::new().limit(1)).await?;
        let mut co = if message.len() == 1 {Some(message.get(0).unwrap())} else {None};
        
        while let Some(message) = co  {
            let messages = channel.messages(ctx.http(), GetMessages::new().before(message.id).limit(100)).await?;
            messages.clone().iter().for_each(|x: &Message| {if u.id == x.author.id {total_messages.push(x)}});
            co = if messages.clone().len() > 0 {Some(messages.clone().get(messages.clone().len() - 1).unwrap())} else {None}
        }
    }
    total_messages.clone().into_iter().for_each(|f| {total_words += utils::count_words(f.clone().content)});

    ctx.say(format!("{} wrote {} words since they arrived",u.name,total_words)).await?;
    Ok(())
}
