fn main() {
    let mut v = Vec::new();
    // push() yeni elemanı sona ekler
    v.push(1);
    v.push(2);
    assert_eq!(v, vec![1, 2]);

    // pop() elemanları sondan çıkarır
    assert_eq!(v.pop().unwrap(), 2);
    assert_eq!(v, vec![1]);
    assert_eq!(v.pop().unwrap(), 1);
    assert_eq!(v, vec![]);
    assert_eq!(v.pop().is_none(), true);
}
