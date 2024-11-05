use poise::{
    samples::HelpConfiguration,
    serenity_prelude::{Error, GetMessages, User},
    Context,
};

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

#[poise::command(prefix_command, slash_command)]
pub async fn count_user_words(
    ctx: Context<'_, Data, Error>,
    #[description = "selected user"] user: Option<User>,
) -> Result<(), Error> {
    println!("Ok we're thinking");
    ctx.defer().await?;
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let channels = ctx.guild_id().unwrap().channels(ctx.http()).await?;
    let ignored_channels = vec!["".to_string()];
    let mut total_words = 0;
    let mut total_messages = Vec::new();
    println!("Got the channels..");
    for (id, channel) in channels {
        println!("Channel {}", id.to_string());
        if ignored_channels.contains(&id.to_string()) {
            continue;
        }
        let message = channel
            .messages(ctx.http(), GetMessages::new().limit(1))
            .await?;
        let mut co = if message.len() == 1 {
            Some(message.get(0).unwrap().clone())
        } else {
            None
        };
        let mut last_id = "".to_string();
        while let Some(ref message) = co {
            println!(
                "Block of 100 message {} so far {} last id",
                &total_messages.len(),
                &message.id
            );
            let messages_raw = channel
                .messages(ctx.http(), GetMessages::new().before(message.id).limit(100))
                .await;

            let messages = match messages_raw {
                Err(e) => {
                    println!("{:#?}", e);
                    println!("{:#?}", message);
                    if last_id != message.id.to_string() {
                        last_id = message.id.to_string();
                        continue;
                    } else {
                        break;
                    }
                }
                Ok(e) => e,
            };

            last_id = message.id.to_string();

            println!("Ok that was {} messages", &messages.len());

            let slice = messages.clone().leak();

            for ele in slice {
                if u.id == ele.author.id {
                    total_messages.push(ele);
                }
            }

            if messages.len() > 0 {
                co = Some(messages.get(messages.len() - 1).unwrap().clone());
            } else {
                co = None;
            }
        }
    }
    println!("Finished!");
    total_messages
        .into_iter()
        .for_each(|f| total_words += utils::count_words(f.clone().content));
    //.for_each(|f| println!("Message : {:#?}", f));

    ctx.say(format!(
        "{} wrote {} words since they arrived",
        u.name, total_words
    ))
    .await?;
    Ok(())
}
