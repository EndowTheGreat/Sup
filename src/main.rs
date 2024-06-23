use clap::Parser;
use crossbeam::thread;
use inline_colorization::{color_cyan, color_reset, color_white};
use ssh2::Session;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::{error::Error, net::TcpStream, path::Path, path::PathBuf};
use toml::Value;
pub mod config;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{color_white}[{color_cyan}Sup{color_white}]{color_reset} {}", format!($($arg)*));
    };
}

macro_rules! elog {
    ($($arg:tt)*) => {
        eprintln!("{color_white}[{color_cyan}Sup{color_white}]{color_reset} {}", format!($($arg)*));
    };
}

fn setup_ssh_session(config: &Value) -> Result<Session, Box<dyn Error>> {
    let username = config
        .get("username")
        .and_then(Value::as_str)
        .ok_or("Missing username in config")?;
    let key_file = config
        .get("key_file")
        .and_then(Value::as_str)
        .ok_or("Missing key_file in config")?;
    let server = config
        .get("server")
        .and_then(Value::as_str)
        .ok_or("Missing server in config")?;
    let port = config
        .get("port")
        .and_then(Value::as_integer)
        .ok_or("Missing server in config")?;
    let mut session = Session::new()?;
    session.set_tcp_stream(TcpStream::connect((server, port as u16))?);
    session.handshake()?;
    session.userauth_pubkey_file(username, None, Path::new(key_file), None)?;
    if session.authenticated() {
        Ok(session)
    } else {
        Err("SSH authentication failed".into())
    }
}

fn upload_sftp(
    session: &Session,
    sftp: &ssh2::Sftp,
    directory: &Path,
    remote_dir: &Path,
    ignore_dotfiles: bool,
    skip: &[PathBuf],
) -> Result<(), Box<dyn Error>> {
    if sftp.opendir(remote_dir).is_err() {
        sftp.mkdir(remote_dir, 0o755)?;
        log!("Created remote directory: {}", remote_dir.display());
    }
    let entries: Vec<_> = fs::read_dir(directory)?.filter_map(Result::ok).collect();
    thread::scope(|s| {
        for entry in entries {
            let path = entry.path();
            let file_name = entry.file_name();
            if skip.contains(&path) {
                log!("Skipping: {}", path.display());
                continue;
            }
            s.spawn(move |_| {
                if path.is_dir() {
                    if ignore_dotfiles
                        && path.file_name().unwrap().to_str().unwrap().starts_with(".")
                    {
                        log!("Skipping Dotfile: {}", path.display());
                    } else {
                        if let Err(e) = upload_sftp(
                            &session,
                            &sftp,
                            &path,
                            &remote_dir.join(&file_name),
                            ignore_dotfiles,
                            skip,
                        ) {
                            elog!("Failed to upload directory {}: {}", path.display(), e);
                        }
                    }
                } else if path.is_file() {
                    if let Err(e) = (|| -> Result<(), Box<dyn Error>> {
                        let mut buffer = Vec::new();
                        File::open(&path)?.read_to_end(&mut buffer)?;
                        sftp.create(&remote_dir.join(&file_name))?
                            .write_all(&buffer)?;
                        log!("Uploaded file: {}", &path.display());
                        Ok(())
                    })() {
                        elog!("Failed to upload file {}: {}", path.display(), e);
                    }
                }
            });
        }
    })
    .unwrap();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = config::Args::parse();
    let config = config::read_config(".sup/config.toml")?;
    let session = setup_ssh_session(&config)?;
    let skip: Vec<PathBuf> = args.skip.split(',').map(PathBuf::from).collect();
    let sftp = session.sftp()?;
    upload_sftp(
        &session,
        &sftp,
        Path::new(&args.directory),
        Path::new(&args.remote_dir),
        args.ignore,
        &skip,
    )?;
    log!("Upload complete.");
    Ok(())
}
