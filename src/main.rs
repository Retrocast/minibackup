use std::fs;

mod config;
mod output;
mod sources;

fn main() {
    let cfg = config::load();
    let mut options = zip::write::FullFileOptions::default()
        .compression_method(zip::CompressionMethod::Zstd)
        .compression_level(Some(1));
    let password;
    if cfg.archive.encrypt {
        password = cfg.archive.password.unwrap_or_else(|| {
            passterm::prompt_password_tty(Some("Enter archive password:")).unwrap()
        });
        options = options.with_aes_encryption(zip::AesMode::Aes256, &password);
    }
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&cfg.archive.dest)
        .unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for x in cfg.sources {
        sources::archive_source(x, &mut zip, options.clone());
    }
    zip.finish().unwrap();
    if let Ok(meta) = std::fs::metadata(&cfg.archive.dest) {
        println!(
            "📁 Final archive size - {}",
            output::format_bytes(meta.len() as f64)
        )
    }
}
