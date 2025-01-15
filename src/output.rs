use std::io::{stdout, Write};

fn flush() {
    let _ = stdout().flush();
}

const SUFFIX: [&'static str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub fn format_bytes(size: f64) -> String {
    if size <= 0.0 {
        return "0 B".to_string();
    }
    let base = size.log10() / 1024_f64.log10();
    format!(
        "{} {}",
        ((1024_f64.powf(base - base.floor()) * 10.0).round() / 10.0),
        SUFFIX[base.floor() as usize]
    )
}

pub fn format_time(time: f64) -> String {
    match time {
        ..1.0 => {
            format!("{:.1}ms", time * 1000.)
        }
        1.0..60.0 => {
            format!("{time:.1}s")
        }
        _ => {
            format!("{}m {}s", (time / 60.).floor(), time % 60.)
        }
    }
}

pub fn print_busy(name: &str) {
    print!("⏳ {name}");
    flush();
}

pub fn print_err(name: &str, err: &str) {
    println!("\r❌ {name}\n└ {err}");
}

pub fn print_done(name: &str, info: &str) {
    println!("\r✅ {name}\n└ {info}");
}
