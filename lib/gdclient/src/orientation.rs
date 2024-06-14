use crate::ParseOrientationError;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Gauche,
    Droite,
    GaucheEtDroite,
}

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

impl Serialize for Orientation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Orientation::Gauche => serializer.serialize_str(" De gauche"),
            Orientation::Droite => serializer.serialize_str(" De droite"),
            Orientation::GaucheEtDroite => serializer.serialize_str(" Les deux"),
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Gauche => write!(f, "de gauche"),
            Orientation::Droite => write!(f, "de droite"),
            Orientation::GaucheEtDroite => write!(f, "les deux"),
        }
    }
}
