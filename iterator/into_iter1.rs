use std::fmt::Debug;

fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

fn main() {
    dump(vec![1, 2, 3]);
    dump(&[1, 2, 3]);
    dump(&mut [1, 2, 3]);
}
