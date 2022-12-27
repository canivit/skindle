use clap::Parser;
use skindle::Args;
use std::process;

fn main() {
    let args = Args::parse();
    if let Err(e) = skindle::run(&args) {
        eprintln!("{e}");
        process::exit(1);
    }
}
