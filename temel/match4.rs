fn good_or_bad(good: bool) -> Result<i32, String> {
    if good { Ok(42) } else { Err("Bad".to_string()) }
}

fn main() {
    println!("{:?}", good_or_bad(true));
    println!("{:?}", good_or_bad(false));

    match good_or_bad(true) {
        Ok(n) => println!("Cool i got {}", n),
        Err(e) => println!("Huh i get {} error", e),
    }

    match good_or_bad(false) {
        Ok(n) => println!("Cool i got {}", n),
        Err(e) => println!("Huh i get {} error", e),
    }
}
