use crate::cli::GdCli;
use clap::Parser;
use gdclient::GdClient;


mod cli;

#[tokio::main]
async fn main() {
    let client = GdClient::new();
    let cli = GdCli::parse();
    println!("{:?}", client.gd(&cli.input).await);
}
