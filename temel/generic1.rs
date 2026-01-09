use std::fmt::Debug;

//Bounded generic types

// Only accept T and U generic types that also implement Debug
fn print_objects<T: Debug, U: Debug>(a: T, b: U) {
    println!("A: {:?} B: {:?}", a, b);
}

fn main() {
    print_objects(13, 44);
    // or annotated explicitly
    print_objects::<usize, u16>(13, 44);
}
