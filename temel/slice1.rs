fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // RangeFull
    let s = &a[..];
    assert_eq!(s, &[1, 2, 3, 4, 5]);

    // RangeFrom
    let s = &a[2..];
    assert_eq!(s, &[3, 4, 5]);

    // RangeTo
    let s = &a[..2];
    assert_eq!(s, &[1, 2]);

    // Range
    let s = &a[2..4];
    assert_eq!(s, &[3, 4]);

    // burada 'x' değerinden 5 tanelik bir array oluşturulması istenilmiştir
    // dizinin tipi [char;5]
    let a = ['x'; 5];
    assert_eq!(&a, &['x', 'x', 'x', 'x', 'x']);
}
