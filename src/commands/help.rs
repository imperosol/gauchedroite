use crate::Context;
use poise::serenity_prelude::Error;

const HELP_MESSAGE: &str = "Je suis un bot destiné à déterminer ce qui est de droite \
et ce qui est de gauche, en utilisant \
l'api du site https://degaucheoudedroite.delemazure.fr/.";

#[poise::command(track_edits, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(HELP_MESSAGE).await.map(|_| ())
}
