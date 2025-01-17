use std::{
    fs::File,
    io::{Read, Write as _},
    process::{Command, Stdio},
};

use zip::{write::FullFileOptions, ZipWriter};

use crate::{config::CommandSourceConfig, output::*};

use super::Source;

pub struct CommandSource {
    pub config: CommandSourceConfig,
}

impl Source for CommandSource {
    fn get_name(&self) -> String {
        self.config.cmd.clone()
    }

    fn execute(
        self,
        zip: &mut ZipWriter<File>,
        options: FullFileOptions,
    ) -> Result<String, String> {
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(self.config.cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Spawning the command failed: {e}"))?;
        let exit = child
            .wait()
            .map_err(|e| format!("Waiting for termination failed: {e}"))?;
        if !exit.success() {
            return Err("Execution failed (nonzero)".to_string());
        }
        let mut buf = String::new();
        child
            .stdout
            .map(|mut x| x.read_to_string(&mut buf).ok())
            .ok_or("Reading stdout failed".to_string())?;
        zip.start_file_from_path(self.config.dest, options)
            .map_err(|e| format!("Creating archive entry failed: {e}"))?;
        zip.write_all(buf.as_bytes())
            .map_err(|e| format!("Writing to archive failed: {e}"))?;
        Ok(format_bytes(buf.len() as f64))
    }
}
