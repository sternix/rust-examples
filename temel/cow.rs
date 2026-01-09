use std::borrow::Cow;

fn get_name() -> String {
    std::env::var("USER").unwrap_or("Whoever you are".to_string())
}

fn get_name_cow() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"))
}

fn get_name_cow_into() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| v.into())
        .unwrap_or("whoever you are".into())
}

fn main() {
    println!("Welcome {}", get_name());
    println!("Welcome {}", get_name_cow());
    println!("Welcome {}", get_name_cow_into());
}
