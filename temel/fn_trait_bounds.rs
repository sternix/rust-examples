use std::ops::{Add, Mul};

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn main() {
    let a = [1, 2, 3, 4];
    let b = [5, 6, 7, 8];
    println!("{}", dot(&a, &b));
}
