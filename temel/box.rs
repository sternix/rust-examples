fn main() {
    // bu stack'te
    let t = (12, "eggs");
    println!("{} {}", t.0, t.1);

    // bu heap'te
    let b = Box::new((12, "eggs"));
    println!("{} {}", b.0, b.1);

    // t şimdi heap'e taşındı
    let b = Box::new(t);
    println!("{} {}", b.0, b.1);
}

/*
burada t'nin tipi (i32,&str), b nin tipi Box<(i32,&str)>

Box::new allocates enough memory to contain the tuple on the heap,
when b goes out of scope the memory freed immediately.
*/
