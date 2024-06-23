use clap::Parser;
use inline_colorization::{color_cyan, color_reset, color_white};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use toml::Value;

use crate::log;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("."), help = "Local directory to upload to the remote server.")]
    pub directory: String,
    #[arg(short, long, help = "Remote directory path for the upload.")]
    pub remote_dir: String,
    #[arg(
        short,
        long,
        default_value_t = String::from(""),
        help = "Files or directories to ignore (separated by ',')."
    )]
    pub skip: String,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Ignore dotfiles when uploading."
    )]
    pub ignore: bool,
}

pub fn read_config(config_path: &str) -> Result<Value, Box<dyn Error>> {
    if !Path::new(config_path).exists() {
        log!("{} does not exist, creating one!", config_path);
        create_default_config(config_path)?;
        log!("Created default configuration file, modify it to get started!");
        std::process::exit(0);
    }
    let mut content = String::new();
    File::open(config_path)?.read_to_string(&mut content)?;
    let config: Value = content.parse()?;
    Ok(config)
}

fn create_default_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(
        Path::new(config_path)
            .parent()
            .ok_or("Invalid config path")?,
    )?;
    let mut config = toml::map::Map::new();
    config.insert(
        "key_file".to_owned(),
        Value::String("/home/user/.ssh/id_rsa.pub".to_owned()),
    );
    config.insert(
        "server".to_owned(),
        Value::String("ssh.example.com".to_owned()),
    );
    config.insert("port".to_owned(), Value::Integer(22));
    config.insert("username".to_owned(), Value::String("root".to_owned()));
    let mut file = File::create(config_path)?;
    file.write_all(toml::to_string(&Value::Table(config))?.as_bytes())?;
    Ok(())
}
