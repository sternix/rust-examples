/*
The unary * operator is used to access the value pointed to by a reference.
As we've seen, Rust automatically follows references when use the . operator to access a field or method,
so the * operator is necessary only when we want to read or write the entire value that the reference points to.
*/

fn print(i: u64) {
    println!("{}", i);
}

fn main() {
    let vals: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7];
    for v in &vals {
        print(*v);
    }
}
