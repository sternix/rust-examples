fn main() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(v.contains(&3), true);
    assert_eq!(v.contains(&5), false);

    assert_eq!(index_of(&v, 3), Some(2));
    assert_eq!(index_of(&v, 5), None);
}

fn index_of(s: &[i32], e: i32) -> Option<usize> {
    s.iter().position(|x| *x == e)
}
