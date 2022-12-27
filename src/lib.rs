use anyhow::{Context, Error, Result};
use clap::Parser;
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    file: PathBuf,
}

#[derive(Deserialize)]
pub struct Config {
    from_email: String,
    to_email: String,
}

pub fn run(args: &Args) -> Result<()> {
    let config = read_config()?;
    Ok(())
}

fn read_config() -> Result<Config> {
    let config_file_path = ProjectDirs::from("canivit", "canivit", "skindle")
        .ok_or("Failed to retrieve a valid home directory path")
        .map_err(Error::msg)?
        .config_dir()
        .join("config.toml");

    let content = fs::read_to_string(&config_file_path).with_context(|| {
        format!(
            "Failed to read the config file {}",
            config_file_path.display(),
        )
    })?;

    let config: Config = toml::from_str(&content).with_context(|| {
        format!(
            "Failed to parse the config file {}",
            config_file_path.display(),
        )
    })?;

    Ok(config)
}

#[cfg(test)]
mod tests {}
