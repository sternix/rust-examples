use std::rc::Rc;

fn main() {
    let five = Rc::new(5);

    {
        let _also_five = Rc::clone(&five);
    }

    // strong referans sayısı
    println!("strong count is {}", Rc::strong_count(&five));
}
