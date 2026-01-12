use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("error occurred while reading from stdin");
    let s = s.trim();
    println!("you typed '{}'", s);
}
