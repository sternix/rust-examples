// https://www.coralbark.net/blog/technology/2020/10/shock-result-rust-faster-than-python-in-one-test-of-file-parsing/

use regex::Regex;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn parse_trace_file(filepath: String) -> std::io::Result<()> {
    let connect_regex: Regex = Regex::new(r"([^T]*)T([^Z]*)Z.*User is authenticated and authorized.*connect=([^\s]*)\s*client=([^\s]*)\s").unwrap();

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut count = 1;

    for lineresult in reader.lines() {
        if let Ok(line) = lineresult {
            //println!("{}", line);

            //Proof of life - so we can tell we haven't hung
            if (count % 20000) == 0 {
                println!("Processing line {}", count);
            }

            count += 1;

            if let Some(caps) = connect_regex.captures(&line) {
                println!("Found a match.");
                println!(
                    "Date    = {}",
                    caps.get(1).map_or("PARSE ERROR", |m| m.as_str())
                );
                println!(
                    "Time    = {}",
                    caps.get(2).map_or("PARSE ERROR", |m| m.as_str())
                );
                println!(
                    "Connect = {}",
                    caps.get(3).map_or("PARSE ERROR", |m| m.as_str())
                );
                println!(
                    "Client  = {}",
                    caps.get(4).map_or("PARSE ERROR", |m| m.as_str())
                );
            }
        }
    }

    Ok(())
}

fn main() {
    let result = parse_trace_file(String::from("/var/tmp/sampletrace.log"));

    if result.is_ok() {
        println!("Yay");
    }
}
