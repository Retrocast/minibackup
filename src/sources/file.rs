use std::{
    fs::File,
    io::{Read, Write as _},
    path::PathBuf,
    time::Instant,
};

use zip::{write::FullFileOptions, ZipWriter};

use crate::output::*;

use super::Success;

fn read_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    if !path.is_file() {
        return Err("File does not exist".to_string());
    }
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {e}"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("Failed to read file: {e}"))?;
    Ok(buf)
}

fn archive_source_inner(
    path: &PathBuf,
    dest: PathBuf,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) -> Result<Success, String> {
    let timer = Instant::now();
    let buf = read_file(path)?;
    zip.start_file_from_path(dest, options)
        .map_err(|e| format!("Failed to create archive entry: {e}"))?;
    zip.write_all(&buf)
        .map_err(|e| format!("Failed to write to archive: {e}"))?;
    Ok(Success {
        elapsed: timer.elapsed().as_secs_f64(),
        info: format_bytes(buf.len() as f64),
    })
}

pub fn archive_source(
    path: &PathBuf,
    dest: PathBuf,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) {
    let name = &path.display().to_string();
    print_busy(name);
    match archive_source_inner(path, dest, zip, options) {
        Ok(x) => print_done(&format!("{name} ({})", format_time(x.elapsed)), &x.info),
        Err(e) => print_err(name, &e),
    }
}
