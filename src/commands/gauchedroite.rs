use crate::Context;
use poise::serenity_prelude::Error;

#[poise::command(track_edits, slash_command)]
pub async fn gauche_ou_droite(ctx: Context<'_>, input: String) -> Result<(), Error> {
    todo!()
}
