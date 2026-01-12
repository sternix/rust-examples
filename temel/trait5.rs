trait Animal {
    fn new(name: &str) -> Self;
    fn name(&self) -> &str;
    fn noise(&self) -> &str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Sheep {
    naked: bool,
    name: String,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked", self.name);
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &str) -> Sheep {
        Sheep {
            name: name.to_string(),
            naked: false,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn noise(&self) -> &str {
        if self.is_naked() {
            "baaaah?"
        } else {
            "baaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut me = Sheep::new("Sheep");
    me.talk();
    me.shear();
    me.talk();

    let mut me: Sheep = Animal::new("Animal");
    me.talk();
    me.shear();
    me.talk();
}
