enum Animal {
    Dog(String),
    Cat { name: String, age: u8 },
    Bird,
}

impl Animal {
    fn name(&self) -> String {
        match self {
            Animal::Dog(name) => name.clone(),
            Animal::Cat { name, .. } => name.clone(),
            Animal::Bird => String::from("kuş"),
        }
    }

    fn age(&self) -> u8 {
        match self {
            Animal::Cat { age, .. } => *age,
            _ => 1,
        }
    }
}

fn print_animal(animal: &Animal) {
    println!("Adı : {}, yaşı: {}", animal.name(), animal.age())
}

fn main() {
    let dog = Animal::Dog(String::from("çomar"));
    let cat = Animal::Cat {
        name: String::from("tekir"),
        age: 3,
    };

    let bird = Animal::Bird;

    print_animal(&dog);
    print_animal(&cat);
    print_animal(&bird);
}
