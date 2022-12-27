use clap::Parser;
use directories::ProjectDirs;
use serde::Deserialize;
use std::error::Error;
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

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let config = read_config()?;
    Ok(())
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    let config_file_path = ProjectDirs::from("canivit", "canivit", "skindle")
        .ok_or("Failed to retrieve a valid home directory path")?
        .config_dir()
        .join("config.toml");

    let content = fs::read_to_string(&config_file_path).map_err(|e| {
        format!(
            "Failed to read the config file {}: {}",
            config_file_path.display(),
            e
        )
    })?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse the config file {}: {}",
            config_file_path.display(),
            e
        )
    })?;

    Ok(config)
}

#[cfg(test)]
mod tests {}
