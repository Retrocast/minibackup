use std::{
    fs::File,
    io::{Read, Write as _},
    path::PathBuf,
    time::Instant,
};

use ignore::{overrides::OverrideBuilder, WalkBuilder};
use zip::{write::FullFileOptions, ZipWriter};

use crate::output::*;

use super::Success;

fn archive_source_inner(
    path: &PathBuf,
    dest: PathBuf,
    respect_gitignore: bool,
    skip_hidden: bool,
    exclude: Vec<String>,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) -> Result<Success, String> {
    if !path.is_dir() {
        return Err("Directory does not exist".to_string());
    }
    let mut _builder = WalkBuilder::new(path);
    let mut builder = _builder.standard_filters(false).hidden(skip_hidden);
    if respect_gitignore {
        builder = builder.git_ignore(true).require_git(false);
    }
    if exclude.len() > 0 {
        let mut overrides = OverrideBuilder::new(path);
        for glob in exclude {
            // Why did library authors decide it is good idea to invert "!"?
            overrides.add(&format!("!{glob}")).unwrap();
        }
        builder = builder.overrides(overrides.build().unwrap())
    }
    let mut total_files = 0;
    let mut total_bytes = 0;
    let mut total_errors = 0;
    let mut buf = vec![];
    let timer = Instant::now();
    // TODO: Add proper error handling
    for result in builder.build() {
        match result {
            Ok(entry) => {
                let entry_path = entry.path();
                let zip_path = dest.join(entry_path.strip_prefix(path).unwrap());
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
    Ok(Success {
        elapsed: timer.elapsed().as_secs_f64(),
        info: format!(
            "{} ({total_files} files) / {total_errors} errors",
            format_bytes(total_bytes as f64)
        ),
    })
}

pub fn archive_source(
    path: &PathBuf,
    dest: PathBuf,
    respect_gitignore: bool,
    skip_hidden: bool,
    exclude: Vec<String>,
    zip: &mut ZipWriter<File>,
    options: FullFileOptions,
) {
    let name = &path.display().to_string();
    print_busy(name);
    match archive_source_inner(
        path,
        dest,
        respect_gitignore,
        skip_hidden,
        exclude,
        zip,
        options,
    ) {
        Ok(x) => print_done(&format!("{name} ({})", format_time(x.elapsed)), &x.info),
        Err(e) => print_err(name, &e),
    }
}
