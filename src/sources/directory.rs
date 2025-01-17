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
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut total_errors = 0;
        let mut buf = vec![];
        // TODO: Add proper error handling
        for result in builder.build() {
            match result {
                Ok(entry) => {
                    let entry_path = entry.path();
                    let zip_path = dest.join(entry_path.strip_prefix(&self.config.path).unwrap());
                    if entry_path.is_dir() {
                        zip.add_directory_from_path(zip_path, options.clone())
                            .unwrap();
                    } else if entry_path.is_file() {
                        zip.start_file_from_path(zip_path, options.clone()).unwrap();
                        let mut f = File::open(entry_path).unwrap();
                        f.read_to_end(&mut buf).unwrap();
                        zip.write_all(&buf).unwrap();
                        total_files += 1;
                        total_bytes += buf.len();
                        buf.clear();
                    }
                }
                Err(_) => total_errors += 1,
            }
        }
        Ok(format!(
            "{} ({total_files} files) / {total_errors} errors",
            format_bytes(total_bytes as f64)
        ))
    }
}
