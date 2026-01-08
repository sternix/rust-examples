// LinkedList
// The main advantage of LinkedList over VecDeque is that combining two lists is very fast. list.append(&mut list2), the method that moves all elements from one list to another, only involves changing a few pointers, which can be done in constant time. The append methods of Vec and VecDeque sometimes have to move many values from one heap array to another.

use std::collections::LinkedList;

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(1);
    assert_eq!(ll.front().unwrap(), &1);

    ll.push_front(2);
    assert_eq!(ll.front().unwrap(), &2);
    assert_eq!(ll.back().unwrap(), &1);

    ll.push_back(3);
    ll.push_back(4);

    assert_eq!(ll.back().unwrap(), &4);

    assert_eq!(ll.pop_front().unwrap(), 2);
    assert_eq!(ll.pop_front().unwrap(), 1);

    assert_eq!(ll.pop_back().unwrap(), 4);
    assert_eq!(ll.pop_back().unwrap(), 3);

    ll.push_back(1);

    if let Some(e) = ll.front_mut() {
        *e = 28;
    }

    assert_eq!(ll.front().unwrap(), &28);

    *(ll.back_mut().unwrap()) = 1;
    assert_eq!(ll.back().unwrap(), &1);

    for e in &ll {
        println!("{}", e);
    }
}
