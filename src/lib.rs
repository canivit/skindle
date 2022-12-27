use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    file: PathBuf,
}

#[cfg(test)]
mod tests {}
