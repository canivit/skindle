use anyhow::{Context, Error, Ok, Result};
use clap::Parser;
use directories::ProjectDirs;
use lettre::{
    message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
pub struct Args {
    ebook_file: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    from_address: String,
    to_address: String,
    convert_to_mobi: bool,
}

struct FileInfo {
    path: PathBuf,
    name: String,
}

impl FileInfo {
    fn from_path(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .map(|path| path.to_string_lossy().to_string())
            .ok_or(Error::msg(format!(
                "File path \"{}\" does not have a file name",
                path.display()
            )))?;

        Ok(FileInfo {
            path: path.to_path_buf(),
            name,
        })
    }
}

pub fn run(args: &Args) -> Result<()> {
    let ebook_file = FileInfo::from_path(&args.ebook_file)?;
    let config = read_config()?;

    let ebook_file = if config.convert_to_mobi {
        convert_to_mobi(&ebook_file)?
    } else {
        ebook_file
    };

    let email = build_email(&config.from_address, &config.to_address, &ebook_file)?;
    send_email(
        &email,
        &config.smtp_server,
        &config.smtp_username,
        &config.smtp_password,
    )?;

    if config.convert_to_mobi {
        delete_temp_mobi_file(&ebook_file.path)?;
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

fn build_email(from_address: &str, to_address: &str, ebook_file: &FileInfo) -> Result<Message> {
    let from = from_address.parse::<Mailbox>().with_context(|| {
        format!(
            "From address \"{}\" is not a valid email address",
            from_address,
        )
    })?;

    let to = to_address
        .parse::<Mailbox>()
        .with_context(|| format!("To address \"{}\" is not a valid email address", to_address,))?;

    let attachment = build_attachment(ebook_file)?;

    Message::builder()
        .from(from)
        .to(to)
        .subject(&ebook_file.name)
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::plain((&ebook_file.name).to_string()))
                .singlepart(attachment),
        )
        .context("Failed to build email")
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
        .context("Sending email failed")
        .and_then(check_reponse)
}

fn build_attachment(ebook_file: &FileInfo) -> Result<SinglePart> {
    let content = fs::read(&ebook_file.path).with_context(|| {
        format!(
            "Failed to read the contents of the ebook file \"{}\"",
            ebook_file.path.display()
        )
    })?;
    let attachment = Attachment::new(ebook_file.name.to_string())
        .body(content, ContentType::parse("application/pdf")?);
    Ok(attachment)
}

fn convert_to_mobi(ebook_file: &FileInfo) -> Result<FileInfo> {
    let dest = env::temp_dir()
        .join(&ebook_file.name)
        .with_extension("mobi");

    let output = Command::new("ebook-convert")
        .arg(&ebook_file.path)
        .arg(&dest)
        .output()
        .with_context(|| {
            concat!(
                "Failed to launch \"ebook-convert\" to convert the ebook file to mobi.",
                " Make sure calibre is installed."
            )
        })?;

    if output.status.success() {
        Ok(FileInfo::from_path(&dest)?)
    } else {
        Err(Error::msg("Failed to convert the ebook file to mobi."))
    }
}

fn delete_temp_mobi_file(path: &Path) -> Result<()> {
    fs::remove_file(path).with_context(|| {
        format!(
            "Failed to delete temp mobi file \"{}\"",
            path.to_string_lossy()
        )
    })
}

fn check_reponse(response: Response) -> Result<()> {
    if response.is_positive() {
        Ok(())
    } else {
        let response_msg = response.message().collect::<Vec<&str>>().join("\n");
        Err(Error::msg(response_msg))
    }
}
