fn get(x: Option<&str>) -> String {
    match x {
        Some(s) if s == "fizz" => format!("have fizz"),
        Some(s) => format!("no fizz just {}", s),
        None => format!("a dud"),
    }
}

fn main() {
    println!("{}", get(Some("fizz")));
    println!("{}", get(Some("abc")));
    println!("{}", get(None));
}
