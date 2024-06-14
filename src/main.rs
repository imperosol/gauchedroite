use crate::cli::GdCli;
use clap::Parser;
use gdclient::GdClient;

mod cli;

#[tokio::main]
async fn main() {
    let client = GdClient::new();
    let cli = GdCli::parse();

    match client.gd(&cli.input).await {
        Ok(res) => println!("{}, c'est {}", &cli.input, res),
        Err(e) => eprintln!("{e}"),
    }
}
