fn main() {
    #[derive(Debug)]
    struct MyType(i32);

    println!("The struct has type {:?}", MyType(19));
}
