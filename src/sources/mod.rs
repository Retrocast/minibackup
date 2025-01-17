use std::{fs::File, time::Instant};

use command::CommandSource;
use directory::DirectorySource;
use file::FileSource;
use zip::{write::FullFileOptions, ZipWriter};

use crate::{config::SourceConfig, output};

pub mod command;
pub mod directory;
pub mod file;

trait Source {
    fn get_name(&self) -> String;
    fn execute(self, zip: &mut ZipWriter<File>, options: FullFileOptions)
        -> Result<String, String>;
}

fn archive_source_inner<S: Source>(source: S, zip: &mut ZipWriter<File>, options: FullFileOptions) {
    let name = source.get_name();
    output::print_busy(&name);
    let timer = Instant::now();
    let result = source.execute(zip, options);
    let elapsed = timer.elapsed().as_secs_f64();
    match result {
        Ok(x) => output::print_done(&format!("{name} ({})", output::format_time(elapsed)), &x),
        Err(e) => output::print_err(&name, &e),
    }
}

pub fn archive_source(source: SourceConfig, zip: &mut ZipWriter<File>, options: FullFileOptions) {
    match source {
        SourceConfig::Command(config) => {
            archive_source_inner(CommandSource { config }, zip, options)
        }
        SourceConfig::Directory(config) => {
            archive_source_inner(DirectorySource { config }, zip, options)
        }
        SourceConfig::File(config) => archive_source_inner(FileSource { config }, zip, options),
    };
}
