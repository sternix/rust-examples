fn main() {
    let v = vec![1, 2, 3, 4];
    let e: Vec<i32> = vec![];

    assert_eq!(v.len(), 4);
    assert_eq!(e.is_empty(), true);
}
