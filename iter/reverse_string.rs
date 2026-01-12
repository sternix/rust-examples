// reverse string

fn main() {
    let s = "Hello";
    let t = s.chars().rev().collect::<String>();

    println!("{t}");
}
