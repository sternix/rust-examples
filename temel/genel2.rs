use std::collections::HashSet;

fn main() {
    let mut v = vec![1, 2, 3, 4];

    // retain() - tutmak, kaybetmemek
    // removes all elements that don't pass given test
    // test argument is a function or closure that implements
    // FnMut(&T) -> bool
    // For each element of vec, this calls test(&element), and if it returns false, the element is removed from the vector and droppped
    v.retain(|e| e % 2 == 0);
    assert_eq!(v, vec![2, 4]);

    // retain'le aynı işi yapıyor
    let mut v = vec![1, 2, 3, 4];
    v = v.into_iter().filter(|e| e % 2 == 0).collect();
    assert_eq!(v, vec![2, 4]);

    // dedup()
    // drops repeated elements. it scans vec for places where adjacent elements are equal and drops the extra equal values, so that only one is left:
    let mut byte_vec = b"Misssssssssssissippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    // tum karakterleri uniq olmasını istersek
    let mut v = b"Misssssssssssissippi".to_vec();
    v.sort();
    v.dedup();
    assert_eq!(&v, b"Mips");

    // diğer bir yol
    // retain ve HashSet
    let mut v = b"Misssssssssssissippi".to_vec();
    let mut seen = HashSet::new();
    v.retain(|r| seen.insert(*r));
    assert_eq!(&v, b"Misp");

    let mut v = vec![1, 1, 2, 2, 3, 3, 4, 4];
    v.dedup_by(|e1, e2| e1 == e2);
    assert_eq!(v, vec![1, 2, 3, 4]);

    #[derive(Debug, PartialEq)]
    struct X {
        y: i32,
    }

    let mut v = vec![
        X { y: 1 },
        X { y: 1 },
        X { y: 2 },
        X { y: 2 },
        X { y: 3 },
        X { y: 3 },
        X { y: 4 },
        X { y: 4 },
    ];

    // istediğimiz bir alana göre dedup
    v.dedup_by_key(|x| x.y);
    assert_eq!(v, vec![X { y: 1 }, X { y: 2 }, X { y: 3 }, X { y: 4 },]);
}
