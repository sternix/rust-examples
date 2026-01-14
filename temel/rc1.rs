/*
Rc Reference Count

For any type T, an Rc<T> value is a pointer to a heap allocated T that has had a reference count affxed to it
Cloning an Rc<T> value does not copy the T; instead it simply creates another pointer to it, and implements the reference count

Each of the three Rc<String> pointers is referring to he same block of memory, which holds a reference count and space for the String. The usual ownership rules apply to the Rc pointers themselves, and when the last extant Rc is dropped, Rust drops the String as well.

You can use any of String's usual methods directly on an Rc<String>
*/

use std::rc::Rc;

fn print(s: Rc<String>) {
    println!("{}", s);
}

fn main() {
    let s = Rc::new("test".to_string());
    let t = s.clone();
    let u = s.clone();

    print(t);

    println!("{}", u);

    // başka bir test
    let s = Rc::new("test".to_string());
    assert!(s.contains("st"));
    assert_eq!(s.find("st"), Some(2));
}
