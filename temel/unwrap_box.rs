// you can actually "unwrap" a box that you own by using `*`
// This can be useful when you have recursive types

fn main() {
    let foo: Box<Vec<u32>> = Box::new(Vec::new());
    let _bar: Vec<u32> = *foo;
}
