use clap::{Parser};

/// Interagit avec l'api degaucheoudedroite pour savoir
/// si quelque chose est de gauche ou de droite.
#[derive(Parser)]
#[command(version, about)]
pub struct GdCli {
    // #[command(subcommand)]
    // feedback: Option<GdSubcommand>,
    /// L'objet pour lequel on veut savoir si c'est de droite ou de gauche
    pub input: String,
}

// #[derive(Subcommand)]
// enum GdSubcommand {
//     ///
//     Agree,
//     Disagree,
// }
