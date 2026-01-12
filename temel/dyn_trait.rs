// aynı Trait ama farklı impl için Box<dyn Trait> kullanılıyor

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

fn random_animal(a: bool) -> Box<dyn Animal> {
    if a {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let animal = random_animal(true);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
    let animal = random_animal(false);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
