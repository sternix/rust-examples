// trait'ten trait göndermek

trait Trait {
    fn howdy(&self);
}

impl Trait for Friend {
    fn howdy(&self) {
        println!("Howdy!");
    }
}

trait Foo {
    fn bar() -> Box<dyn Trait>;
}

struct Friend;

impl Foo for () {
    fn bar() -> Box<dyn Trait> {
        Box::new(Friend)
    }
}

fn main() {
    <() as Foo>::bar().howdy();
}
