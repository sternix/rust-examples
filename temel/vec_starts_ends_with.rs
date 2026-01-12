fn main() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(v.starts_with(&[1, 2]), true);
    assert_eq!(v.starts_with(&[2, 3]), false);

    assert_eq!(v.ends_with(&[3, 4]), true);
    assert_eq!(v.ends_with(&[1, 2]), false);
}
