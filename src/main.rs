use crate::cli::GdCli;
use clap::Parser;


mod cli;

#[tokio::main]
async fn main() {
    let cli = GdCli::parse();
    println!("{:?}", &cli.input);
}
