fn main() {
    let template = format!(
        "
            server {{
                    hostname: \"{hostname}\",
                    address: \"{address}\",
                    username: \"{username}\",
                    password: \"{password}\",
            }}",
        hostname = "db.example.com",
        address = "192.168.1.2",
        username = "admin",
        password = "secret"
    );
    println!("{}", template)
}
