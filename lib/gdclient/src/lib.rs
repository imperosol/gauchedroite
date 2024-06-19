pub mod cache;
mod client;
mod orientation;
mod payload;

pub use client::GdClient;
pub use orientation::Orientation;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Couldn't parse political orientation.")]
pub struct ParseOrientationError;

/// Les erreurs qui peuvent arriver lors de l'interaction avec l'API.
#[derive(Debug, Error)]
pub enum GdClientError {
    /// Le problème est arrivé au niveau de reqwest.
    /// Ca veut probablement dire que la requête n'a
    /// tout simplement pas pu être envoyée ou que la réponse
    /// n'a pas pu être reçue,
    /// sans doute parce que le serveur est inaccessible.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Erreur renvoyée par le serveur.
    /// Généralement, c'est un code 4XX.
    #[error("Code {code} : {message}")]
    Client { message: String, code: u32 },
}
