use poise::{samples::HelpConfiguration, serenity_prelude::Error, Context};

use crate::Data;

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
