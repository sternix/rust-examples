use std::collections::VecDeque;

/*
VecDeque<T>

std::collections::VecDeque<T> is a deque (pronounced "deck"), a double-ended queue
*/

fn main() {
    let mut d = VecDeque::new();
    d.push_front(1);
    d.push_front(2);

    assert_eq!(Vec::from(d.clone()), vec![2, 1]);

    d.push_back(3);
    d.push_back(4);

    assert_eq!(Vec::from(d.clone()), vec![2, 1, 3, 4]);

    assert_eq!(d.front().unwrap(), &2);
    assert_eq!(d.back().unwrap(), &4);

    assert_eq!(d.pop_front().unwrap(), 2);
    assert_eq!(d.pop_front().unwrap(), 1);

    assert_eq!(d.pop_back().unwrap(), 4);
    assert_eq!(d.pop_back().unwrap(), 3);

    assert_eq!(d.front().is_none(), true);
    assert_eq!(d.back().is_none(), true);

    d.push_back(1);

    if let Some(e) = d.front_mut() {
        *e = 28;
    }

    assert_eq!(d.front().unwrap(), &28);

    *(d.back_mut().unwrap()) = 1;
    assert_eq!(d.back().unwrap(), &1);

    assert_eq!(d.get(0).unwrap(), &1);
    *(d.get_mut(0).unwrap()) = 28;
    assert_eq!(d.get(0).unwrap(), &28);

    let v = vec![1, 2, 3, 4];
    let d = VecDeque::from(v);

    assert_eq!(d.front().unwrap(), &1);
    assert_eq!(d.back().unwrap(), &4);

    for e in &d {
        println!("{}", e);
    }
}
