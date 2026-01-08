fn main() {
    let mut v = vec!["x", "y", "z"];

    // returns a reference to the first element of vector, is any
    // return type is Option<&T>, so the return value is None if vector is empty
    if let Some(first) = v.first() {
        println!("First Element: {}", first);
    }

    // similar to first but returns a reference to last element
    if let Some(last) = v.last() {
        println!("Last element: {}", last);
    }

    // returns Option<&T>
    if let Some(item) = v.get(1) {
        println!("Element at index 1 is {}", item);
    }

    assert_eq!(v.get(3), None);

    // first_mut(), last_mut(), get_mut(index)
    // are variations of these that borrows mut references
    {
        let last = v.last_mut().unwrap();
        assert_eq!(*last, "z");
        *last = "w";
    }
    assert_eq!(v, ["x", "y", "w"]);

    // to_vec() this method is available only if the elements are cloneable, that is where T: Clone
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.to_vec(), vec![1, 2, 3, 4, 5]);
    assert_eq!(a[0..5].to_vec(), vec![1, 2, 3, 4, 5]);
    assert_eq!(a[0..=4].to_vec(), vec![1, 2, 3, 4, 5]);
}
