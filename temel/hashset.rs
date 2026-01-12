use std::collections::HashSet;

fn main() {
    let mut s = HashSet::new();
    s.insert(1);
    s.insert(2);
    s.insert(3);
    assert_eq!(s.insert(3), false);
    assert_eq!(s.contains(&3), true);
    assert_eq!(s.remove(&9), false);

    // Option<&T>
    assert_eq!(s.get(&2).unwrap(), &2);

    // like remove(&value), but it returns the removed value, if any
    // Option<T>
    assert_eq!(s.take(&2).unwrap(), 2);

    // like insert() but if set already contains a value that's equal to value, this replaces and returns the old value
    // Option<T>
    assert_eq!(s.replace(1).unwrap(), 1);
    assert_eq!(s.replace(4).is_none(), true);
}
