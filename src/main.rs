use std::{fs, path::PathBuf};

use config::Source;

mod config;
mod output;
mod sources;

fn main() {
    let cfg: config::Config =
        toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
    let mut options = zip::write::FullFileOptions::default()
        .compression_method(zip::CompressionMethod::Zstd)
        .compression_level(Some(1));
    let password;
    if cfg.archive.encrypt {
        password = passterm::prompt_password_tty(Some("Enter archive password:")).unwrap();
        options = options.with_aes_encryption(zip::AesMode::Aes256, &password);
    }
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(cfg.archive.dest)
        .unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for x in cfg.sources {
        let opts = (&options).clone();
        match x {
            Source::Command { cmd, dest } => {
                sources::command::archive_source(&cmd, dest, &mut zip, opts);
            }
            Source::Directory {
                path,
                dest,
                respect_gitignore,
                skip_hidden,
                exclude,
            } => {
                sources::directory::archive_source(
                    &path,
                    dest.unwrap_or_else(|| PathBuf::from(path.file_name().unwrap())),
                    respect_gitignore,
                    skip_hidden,
                    exclude.unwrap_or_else(|| vec![]),
                    &mut zip,
                    opts,
                );
            }
            Source::File { path, dest } => {
                sources::file::archive_source(
                    &path,
                    dest.unwrap_or_else(|| PathBuf::from(path.file_name().unwrap())),
                    &mut zip,
                    opts,
                );
            }
        }
    }
    zip.finish().unwrap();
}
