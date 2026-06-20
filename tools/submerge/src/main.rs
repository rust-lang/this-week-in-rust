use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    submerge::run(submerge::Args::parse()).await
}
