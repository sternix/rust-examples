use std::fmt::Arguments;
use std::fs::OpenOptions;
use std::io::Write;

fn write_log_entry(entry: Arguments) {
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log-file-name")
        .expect("failed to open log file !");

    log_file.write_fmt(entry).expect("failed to write to log");
}

macro_rules! log {
    ($format:tt, $($arg:expr),*) => (
        write_log_entry(format_args!($format, $($arg),*))
    )
}

fn main() {
    let a = 10;
    write_log_entry(format_args!("pointer of a {:p}\n", &a));

    // macro'lu hali
    log!("pointer of a {:p}\n", &a);
}

// pointer of a 0xffbfe66c
