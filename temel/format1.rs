fn main() {
    let template = format!(
        "
            server {{
                    hostname: \"{0}\",
                    address: \"{1}\",
                    username: \"{2}\",
                    password: \"{3}\",
            }}",
        "db.example.com", "192.168.1.2", "admin", "secret"
    );
    println!("{}", template)
}
