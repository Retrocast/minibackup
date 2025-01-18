use std::{
    fs::File,
    io::{Read, Write as _},
    path::PathBuf,
};

use ignore::{overrides::OverrideBuilder, WalkBuilder};
use zip::{write::FullFileOptions, ZipWriter};

use crate::{config::DirectorySourceConfig, output::*};

use super::Source;

pub struct DirectorySource {
    pub config: DirectorySourceConfig,
}

impl Source for DirectorySource {
    fn get_name(&self) -> String {
        self.config.path.display().to_string()
    }

    fn execute(
        self,
        zip: &mut ZipWriter<File>,
        options: FullFileOptions,
    ) -> Result<String, String> {
        if !self.config.path.is_dir() {
            return Err("Directory does not exist".to_string());
        }
        let dest = self
            .config
            .dest
            .unwrap_or_else(|| PathBuf::from(self.config.path.file_name().unwrap()));
        let mut _builder = WalkBuilder::new(&self.config.path);
        let mut builder = _builder
            .standard_filters(false)
            .hidden(self.config.skip_hidden);
        if self.config.respect_gitignore {
            builder = builder.git_ignore(true).require_git(false);
        }
        if self.config.exclude.len() > 0 {
            let mut overrides = OverrideBuilder::new(&self.config.path);
            for glob in self.config.exclude {
                // Why did library authors decide it is good idea to invert "!"?
                overrides.add(&format!("!{glob}")).unwrap();
            }
            builder = builder.overrides(overrides.build().unwrap())
        }
        if self.config.max_file_size > 0. {
            let s = self.config.max_file_size as u64;
            builder = builder.filter_entry(move |e| {
                if let Ok(x) = e.metadata() {
                    x.is_dir() || x.len() <= s
                } else {
                    false
                }
            })
        }
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut archival_errors = 0;
        let mut fs_errors = 0;
        let mut buf = vec![];
        // TODO: Add better error handling.
        for result in builder.build() {
            match result {
                Ok(entry) => {
                    let entry_path = entry.path();
                    let zip_path = dest.join(entry_path.strip_prefix(&self.config.path).unwrap());
                    if entry_path.is_dir() {
                        if zip
                            .add_directory_from_path(zip_path, options.clone())
                            .is_err()
                        {
                            archival_errors += 1;
                        }
                    } else if entry_path.is_file() {
                        let mut f = match File::open(entry_path) {
                            Ok(f) => f,
                            Err(_) => {
                                fs_errors += 1;
                                continue;
                            }
                        };
                        if f.read_to_end(&mut buf).is_err() {
                            fs_errors += 1;
                            continue;
                        }
                        if zip.start_file_from_path(zip_path, options.clone()).is_err() {
                            archival_errors += 1;
                            continue;
                        }
                        if zip.write_all(&buf).is_err() {
                            archival_errors += 1;
                            continue;
                        }
                        total_files += 1;
                        total_bytes += buf.len();
                        buf.clear();
                    }
                }
                Err(_) => fs_errors += 1,
            }
        }
        let mut error_text = String::new();
        if archival_errors > 0 {
            error_text += &format!(" / {archival_errors} archival errors");
        }
        if fs_errors > 0 {
            error_text += &format!(" / {fs_errors} filesystem errors");
        }
        Ok(format!(
            "{} ({total_files} files){}",
            format_bytes(total_bytes as f64),
            error_text
        ))
    }
}
