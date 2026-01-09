// https://cglab.ca/~abeinges/blah/rust-generics-and-collections/

fn main() {
    // u32 suffix literal denotes an unsigned 32-bit integer; i16 is signed 16-bit.
    // All other values in the arrays are inferred to be of the same type.
    let a = find_min(vec![5u32, 2, 3, 4]).unwrap();
    let b = find_min(vec![10i16, 20, 30, 40]).unwrap();
    println!("{} {}", a, b);
}

// Dat Code Reuse
fn find_min<T: Ord>(data: Vec<T>) -> Option<T> {
    let mut it = data.into_iter();
    let mut min = match it.next() {
        Some(elem) => elem,
        None => return None,
    };
    for elem in it {
        if elem < min {
            min = elem;
        }
    }
    Some(min)
}
