# Skindle - Send To Kindle
CLI tool to send e-book files to kindle e-readers via email

## Usage
```shell
skindle programming_rust.epub
```

## Configuration
On Linux, create `~/.config/skindle/config.toml` and add the following: 
```toml
smtp_server = "smtp.gmail.com"
smtp_username = "your_email@gmail.com"
smtp_password = "your_password"
from_address = "your_email@gmail.com"
to_address = "your_kindle_email@kindle.com"
convert_to_mobi = true
```

## Motivation
1. I often download CS/programming related ebooks to my laptop running NixOS. Later, if I like a book, I want to continue reading it on my kindle. I wanted to have a simple and quick CLI that can send the ebook files from my laptop to my kindle reader.
2. I wanted to practice Rust by building a CLI.


