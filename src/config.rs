use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub archive: ArchiveConfig,
    pub sources: Vec<Source>,
}

fn default_archive_dest() -> PathBuf {
    PathBuf::from("backup.zip")
}

#[derive(Deserialize, Debug)]
pub struct ArchiveConfig {
    #[serde(default)]
    pub encrypt: bool,
    #[serde(default = "default_archive_dest")]
    pub dest: PathBuf,
}

impl Default for ArchiveConfig {
    fn default() -> Self {
        Self {
            encrypt: false,
            dest: default_archive_dest(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    Command {
        cmd: String,
        dest: PathBuf,
    },
    Directory {
        path: PathBuf,
        dest: Option<PathBuf>,
        #[serde(default = "default_true")]
        respect_gitignore: bool,
        #[serde(default)]
        skip_hidden: bool,
        exclude: Option<Vec<String>>,
    },
    File {
        path: PathBuf,
        dest: Option<PathBuf>,
    },
}
