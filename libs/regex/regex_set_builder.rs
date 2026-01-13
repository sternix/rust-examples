/*

https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
Reads a file named application.log and only outputs the lines containing "version X.X.X", some IP address followed by port 443 (e.g. “192.168.0.1:443”), or a specific warning.
A regex::RegexSetBuilder composes a regex::RegexSet. Since backslashes are very common in regular expressions, using raw string literals makes them more readable.

*/

use regex::RegexSetBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "postfix.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}
