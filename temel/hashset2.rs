use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut a: HashSet<i32> = vec![1_i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2_i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));
    assert_eq!(a.insert(4), false);

    b.insert(5);

    // intersection() bir iteratör döndürüyor
    let mut v = a.intersection(&b).collect::<Vec<&i32>>();
    v.sort();
    assert_eq!(v, vec![&2, &3, &4]);

    // intersection operator & sonuçta kesişim'in HashSet'ini döndürüyor
    let is = &a & &b;
    let mut v = Vec::from_iter(is);
    v.sort();
    assert_eq!(v, vec![2, 3, 4]);

    // union() iterator döndürüyor
    let mut v = Vec::from_iter(a.union(&b));
    v.sort();
    assert_eq!(v, vec![&1, &2, &3, &4, &5]);

    // union operatörü kesişim'in HashSet'ini döndürüyor
    let un = &a | &b;
    let mut v = Vec::from_iter(un);
    v.sort();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);

    // difference metodu bir iteratör döndürüyor
    let diff = a.difference(&b);
    let mut v = Vec::from_iter(diff);
    v.sort();
    assert_eq!(v, vec![&1]);
    // difference operatörü bir HashSet döndürüyor
    let diff = &a - &b;
    let v = Vec::from_iter(diff);
    assert_eq!(v, vec![1]);

    let diff = &b - &a;
    let v = Vec::from_iter(diff);
    assert_eq!(v, vec![5]);

    // intersection'un dışı
    let sd = a.symmetric_difference(&b);
    let mut v = Vec::from_iter(sd);
    v.sort();
    assert_eq!(v, vec![&1, &5]);

    let sd = &a ^ &b;
    let mut v = Vec::from_iter(sd);
    v.sort();
    assert_eq!(v, vec![1, 5]);

    let a: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3]);
    let b: HashSet<i32> = HashSet::from_iter(vec![4, 5, 6]);

    // ortak elemanları yoksa true
    assert_eq!(a.is_disjoint(&b), true);

    let a: HashSet<i32> = HashSet::from_iter(vec![2, 3]);
    let b: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3]);

    // alt küme ise true
    assert_eq!(a.is_subset(&b), true);

    let a: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3, 4]);
    let b: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3]);

    // üst küme mi
    // b'deki tüm elemanlar a'da olmalı
    // a'nın fazladan elemanı olabilir
    assert_eq!(a.is_superset(&b), true);
    let a: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3]);
    let b: HashSet<i32> = HashSet::from_iter(vec![1, 2, 3]);
    assert!(a == b);
}
