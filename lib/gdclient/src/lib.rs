mod client;
mod payload;

use std::str::FromStr;
use thiserror::Error;

pub use client::GdClient;

#[derive(Debug)]
pub enum Orientation {
    Gauche,
    Droite,
    GaucheEtDroite,
}

#[derive(Debug, Error)]
#[error("Couldn't parse political orientation.")]
pub struct ParseOrientationError;

impl FromStr for Orientation {
    type Err = ParseOrientationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " De gauche" => Ok(Orientation::Gauche),
            " De droite" => Ok(Orientation::Droite),
            " Les deux" => Ok(Orientation::GaucheEtDroite),
            _ => Err(ParseOrientationError),
        }
    }
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Code {code} : {message}")]
    Client { message: String, code: u32 },
}
