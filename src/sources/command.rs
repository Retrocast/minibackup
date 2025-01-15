use std::{
    fs::File,
    io::{Read, Write as _},
    path::PathBuf,
    process::{Command, Stdio},
    time::Instant,
};

use zip::{write::FullFileOptions, ZipWriter};

use crate::output::*;

use super::Success;

fn execute_cmd(cmd: &String) -> Result<String, String> {
    let mut command = &mut Command::new("bash");
    command = command
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = command.spawn().map_err(|e| e.to_string())?;
    let exit = child.wait().map_err(|e| e.to_string())?;
    if !exit.success() {
        return Err("Execution failed (nonzero)".to_string());
    }
    let mut buf = String::new();
    child
        .stdout
        .map(|mut x| x.read_to_string(&mut buf).ok())
        .ok_or("Failed to read stdout".to_string())?;
    Ok(buf)
}

fn archive_source_inner(
    cmd: &String,
    dest: PathBuf,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) -> Result<Success, String> {
    let timer = Instant::now();
    let output = execute_cmd(cmd)?;
    zip.start_file_from_path(dest, options)
        .map_err(|e| format!("Failed to create archive entry: {e}"))?;
    zip.write_all(output.as_bytes())
        .map_err(|e| format!("Failed to write to archive: {e}"))?;
    Ok(Success {
        elapsed: timer.elapsed().as_secs_f64(),
        info: format_bytes(output.len() as f64),
    })
}

pub fn archive_source(
    cmd: &String,
    dest: PathBuf,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) {
    print_busy(cmd);
    match archive_source_inner(cmd, dest, zip, options) {
        Ok(x) => print_done(&format!("{cmd} ({})", format_time(x.elapsed)), &x.info),
        Err(e) => print_err(cmd, &e),
    }
}
