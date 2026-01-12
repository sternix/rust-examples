fn print_type<T: ?Sized>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}

fn main() {
    let a = [1, 2];
    let v = vec![1, 2];
    let s = "abc".to_string();
    let st = "Test";

    print_type(&a);
    print_type(&v);
    print_type(&s);
    print_type(&st);
}
