/*
BinaryHeap

std::collections::BinaryHeap

Ord traitini impl eden tipler için,
büyük eleman en üstte
*/

use std::collections::BinaryHeap;

fn main() {
    let mut h = BinaryHeap::new();
    // eleman ekler,
    // her ekleme sırayı yeniden düzenler
    h.push(1);
    h.push(3);
    h.push(2);

    // en üstteki elemanın referansı
    assert_eq!(h.peek().unwrap(), &3);

    // en üstteki elemanı alır
    assert_eq!(h.pop().unwrap(), 3);
    assert_eq!(h.peek().unwrap(), &2);

    let mut h2 = BinaryHeap::from(vec![5, 6, 7, 8]);
    h.append(&mut h2);

    // burada bir sıra ile vermiyor
    println!("for-in Sırasız");
    for e in &h {
        println!("{}", e);
    }

    println!("Sıralı");
    while let Some(e) = h.pop() {
        println!("{}", e);
    }
}
