/*
The . operator can also implicitly borrow a reference to its left operand,
if needed for a method call. For example Vec's sort method takes a mutable
reference to the vector, so the two calls shown here are equivalent.
*/

fn main() {
    let mut v = vec![4567, 1234];
    v.sort();
    println!("{:?}", v);

    let mut v = vec![4567, 1234];
    (&mut v).sort();
    println!("{:?}", v);
}
