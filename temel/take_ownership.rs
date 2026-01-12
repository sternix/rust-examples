fn main() {
    let s = "Test".to_string();

    let take_ownership = move || s;

    println!("{}", take_ownership());
}
