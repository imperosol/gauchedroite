use crate::cli::GdCli;
use clap::Parser;
use gdclient::cache::SqliteCache;
use gdclient::GdClient;

mod cli;

#[tokio::main]
async fn main() {
    let mut client = GdClient::with_cache(SqliteCache::connect());
    let cli = GdCli::parse();

    match client.gd(&cli.input).await {
        Ok(res) => println!("{}, c'est {}", &cli.input, res),
        Err(e) => eprintln!("{e}"),
    }
}
