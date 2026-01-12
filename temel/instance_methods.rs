// metodu ya instance'dan yada struct'a &self parametresi ileterek çağırabiliyoruz

struct Human;

impl Human {
    fn fly(&self) {
        println!("Human Flying");
    }
}

fn main() {
    let h = Human;
    h.fly();

    Human::fly(&h);
}
