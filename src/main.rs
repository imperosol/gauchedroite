use dotenv::dotenv;
use poise::serenity_prelude::{ClientBuilder, Error, GatewayIntents, GuildId};
use std::env;

mod api;
mod commands;

type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        env::var("BOT_TOKEN").expect("Le token doit être donné en variable d'environnement");
    let framework = poise::Framework::builder()
        .setup(|ctx, _, _| {
            Box::pin(async {
                println!("Bot démarré");
                Ok(())
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![commands::help(), commands::gauche_ou_droite()],
            ..Default::default()
        })
        .build();
    ClientBuilder::new(token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
}
