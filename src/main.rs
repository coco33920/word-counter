use std::env;

use commands::commands::{count_user_words, help};
use dotenv::dotenv;
use poise::{
    serenity_prelude::{ClientBuilder, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};

struct Data;

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token =
        env::var("DISCORD_TOKEN").expect("You should have a discord token configurated!");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![help(), count_user_words()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();

    let mut client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .expect("Client creation failed :(");

    client.start().await.unwrap()
}
