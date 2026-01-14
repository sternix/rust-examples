fn get_name() -> String {
    std::env::var("USER").unwrap_or("Whoever you are".to_string())
}

fn main() {
    println!("Welcome {}", get_name());
}
