// pub fn count(self) -> usize

fn main() {
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().count(), 5);

    let v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(v.iter().count(), 6);
}
