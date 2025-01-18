use std::{fs, path::PathBuf, process::exit};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub archive: ArchiveConfig,
    pub sources: Vec<SourceConfig>,
}

fn default_archive_dest() -> PathBuf {
    PathBuf::from("backup.zip")
}

#[derive(Deserialize, Debug)]
pub struct ArchiveConfig {
    #[serde(default)]
    pub encrypt: bool,
    pub password: Option<String>,
    #[serde(default = "default_archive_dest")]
    pub dest: PathBuf,
}

impl Default for ArchiveConfig {
    fn default() -> Self {
        Self {
            encrypt: false,
            password: None,
            dest: default_archive_dest(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_empty() -> Vec<String> {
    vec![]
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SourceConfig {
    Command(CommandSourceConfig),
    Directory(DirectorySourceConfig),
    File(FileSourceConfig),
}

#[derive(Deserialize, Debug)]
pub struct CommandSourceConfig {
    pub cmd: String,
    pub dest: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct DirectorySourceConfig {
    pub path: PathBuf,
    pub dest: Option<PathBuf>,
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
    #[serde(default)]
    pub skip_hidden: bool,
    #[serde(default = "default_empty")]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub max_file_size: f64,
}

#[derive(Deserialize, Debug)]
pub struct FileSourceConfig {
    pub path: PathBuf,
    pub dest: Option<PathBuf>,
}

pub fn load() -> Config {
    let args = std::env::args().collect::<Vec<_>>();
    let path = if args.len() == 2 {
        &args[1]
    } else {
        "config.toml"
    };
    let config = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to read config file: {e}");
            exit(1);
        }
    };
    match toml::from_str(&config) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to parse config file: {}", e.message());
            exit(1);
        }
    }
}
