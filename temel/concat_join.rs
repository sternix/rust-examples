fn main() {
    let s = [[1, 2], [3, 4], [5, 6]].concat();
    assert_eq!(s, vec![1, 2, 3, 4, 5, 6]);

    let v = vec![[1, 2], [3, 4], [5, 6]].concat();
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);

    // concat ile aynı sadece birleştirilecek elemanların arasına bizim verdiğimiz elemanı ekliyor.
    let s = [[1, 2], [3, 4], [5, 6]].join(&0);
    assert_eq!(s, vec![1, 2, 0, 3, 4, 0, 5, 6]);
}
