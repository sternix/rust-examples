use std::rc::Rc;

fn main() {
    let original = Rc::new("hello".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("hello".to_string());

    println!("text: {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p} {:p} {:p}", original, cloned, impostor);
}

/*
text: hello, hello, hello
pointers: 0x21276068 0x21276068 0x21276088

dikkat edilirse original ve cloned'ın adresleri aynı
*/
