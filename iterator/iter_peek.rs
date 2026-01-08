fn main() {
    // peek() ile iterator'ü next()'i çalıştırmadan elemana bakabiliriz,
    // dikkat edilirse &&X şeklinde

    let xs = [1, 2, 3];

    let mut iter = xs.iter().peekable();

    // peek() lets us see into the future
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.next(), Some(&2));

    // we can peek() multiple times, the iterator won't advance
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&3));

    // after the iterator is finished, so is peek()
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);
}
