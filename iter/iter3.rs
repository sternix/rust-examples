fn main() {
    let v = vec![4, 20, 12, 8, 6];

    // bunlar referans dönüyür
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    // bunlar value dönüyor
    let mut iterator = v.into_iter();
    assert_eq!(iterator.next(), Some(4));
    assert_eq!(iterator.next(), Some(20));
    assert_eq!(iterator.next(), Some(12));
    assert_eq!(iterator.next(), Some(8));
    assert_eq!(iterator.next(), Some(6));
    assert_eq!(iterator.next(), None);
}
