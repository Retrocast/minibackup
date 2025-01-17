use std::{
    fs::File,
    io::{Read, Write as _},
    path::PathBuf,
};

use zip::{write::FullFileOptions, ZipWriter};

use crate::{config::FileSourceConfig, output::*};

use super::Source;

pub struct FileSource {
    pub config: FileSourceConfig,
}

impl Source for FileSource {
    fn get_name(&self) -> String {
        self.config.path.display().to_string()
    }

    fn execute(
        self,
        zip: &mut ZipWriter<File>,
        options: FullFileOptions,
    ) -> Result<String, String> {
        if !self.config.path.is_file() {
            return Err("File does not exist".to_string());
        }
        let mut file =
            File::open(&self.config.path).map_err(|e| format!("Failed to open file: {e}"))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Failed to read file: {e}"))?;
        zip.start_file_from_path(
            self.config
                .dest
                .unwrap_or_else(|| PathBuf::from(self.config.path.file_name().unwrap())),
            options,
        )
        .map_err(|e| format!("Creating archive entry failed: {e}"))?;
        zip.write_all(&buf)
            .map_err(|e| format!("Writing to archive failed: {e}"))?;
        Ok(format_bytes(buf.len() as f64))
    }
}
