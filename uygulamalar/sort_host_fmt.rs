use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Host {
    name: String,
    i: String,
    ac: String,
    ip: String,
}

fn main() {
    let file = env::args().nth(1).expect("Dosya adı ?");

    if let Ok(lines) = read_lines(file) {
        let mut hosts: Vec<Host> = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                let parts = line.split(';').collect::<Vec<_>>();
                let i = Host {
                    name: parts[0].into(),
                    i: parts[1].into(),
                    ac: parts[2].into(),
                    ip: parts[3].into(),
                };
                hosts.push(i);
            }
        }

        hosts.sort_by(|a, b| a.ip.cmp(&b.ip));

        for h in hosts {
            println!("{}\t\t{}\t\t{}\t{}", h.name, h.i, h.ac, h.ip);
        }
    } else {
        println!("Dosya açılamadı")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// println!("{: <25}{}\t{: <10}{}", parts[0], parts[1], parts[2], parts[3]);
