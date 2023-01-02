use anyhow::{Context, Error, Ok, Result};
use clap::Parser;
use directories::ProjectDirs;
use lettre::{
    message::Mailbox,
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser)]
pub struct Args {
    ebook_file: PathBuf,
}

#[derive(Deserialize)]
pub struct Config {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    from_address: String,
    to_address: String,
}

pub fn run(args: &Args) -> Result<()> {
    validate_ebook_file(&args.ebook_file)?;
    let config = read_config()?;
    let email = build_email(&config.from_address, &config.to_address, &args.ebook_file)?;
    send_email(
        &email,
        &config.smtp_server,
        &config.smtp_username,
        &config.smtp_password,
    )
}

fn validate_ebook_file(ebook_file: &Path) -> Result<()> {
    ebook_file
        .try_exists()
        .with_context(|| {
            format!(
                "Failed to check if the ebook file \"{}\" exists",
                ebook_file.display()
            )
        })
        .and_then(|exists| {
            if exists {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Ebook file \"{}\" does not exist",
                    ebook_file.display()
                )))
            }
        })?;

    if !ebook_file.is_file() {
        return Err(Error::msg(format!(
            "\"{}\" is not a file",
            ebook_file.display()
        )));
    }

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
            "Failed to read the config file \"{}\"",
            config_file_path.display(),
        )
    })?;

    let config: Config = toml::from_str(&content).with_context(|| {
        format!(
            "Failed to parse the config file \"{}\"",
            config_file_path.display(),
        )
    })?;

    Ok(config)
}

fn build_email(from_address: &str, to_address: &str, ebook_file: &Path) -> Result<Message> {
    let from = from_address.parse::<Mailbox>().with_context(|| {
        format!(
            "From address \"{}\" is not a valid email address",
            from_address,
        )
    })?;

    let to = to_address
        .parse::<Mailbox>()
        .with_context(|| format!("To address \"{}\" is not a valid email address", to_address,))?;

    // ebook_file is already validated so it is safe to call unwrap
    let subject = ebook_file.file_name().unwrap().to_string_lossy();

    Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .body(String::from(""))
        .with_context(|| "Failed to build email")
}

fn send_email(
    email: &Message,
    smtp_server: &str,
    smtp_username: &str,
    smtp_password: &str,
) -> Result<()> {
    let sender = SmtpTransport::starttls_relay(smtp_server)
        .with_context(|| format!("Failed to connect to the SMTP server \"{}\"", smtp_server))?
        .credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_password.to_string(),
        ))
        .build();

    sender
        .send(email)
        .with_context(|| "Sending email failed")
        .and_then(check_reponse)
}

fn check_reponse(response: Response) -> Result<()> {
    if response.is_positive() {
        Ok(())
    } else {
        let response_msg = response.message().collect::<Vec<&str>>().join("\n");
        Err(Error::msg(response_msg))
    }
}

#[cfg(test)]
mod tests {}
