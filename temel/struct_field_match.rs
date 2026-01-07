struct Person {
    name: Option<String>,
    birth: i32,
}

impl Person {
    fn print(&self) {
        match self {
            Person {
                name: Some(name),
                birth,
            } => println!("{} {}", name, birth),
            Person { name: None, birth } => println!("{}", birth),
        }
    }
}

fn main() {
    let p = Person {
        name: Some("adi".to_string()),
        birth: 1234,
    };

    p.print();

    let p = Person {
        name: None,
        birth: 1234,
    };

    p.print();
}
