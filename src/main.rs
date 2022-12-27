use clap::Parser;
use skindle::Args;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    skindle::run(&args)
}
