// pub fn last(self) -> Option<Self::Item>

fn main() {
    let a = [1, 2, 3, 4, 5];

    assert_eq!(a.last().unwrap(), &5);
    assert_eq!(*a.iter().last().unwrap(), 5);
    assert_eq!(a.iter().last().unwrap(), &5);

    let v = vec![1, 2, 3, 4, 5];
    assert_eq!(v.last().unwrap(), &5);
    assert_eq!(v.iter().last().unwrap(), &5);
    assert_eq!(*v.iter().last().unwrap(), 5);
}
