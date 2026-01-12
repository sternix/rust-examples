fn main() {
    let v = [String::from("hello"), String::from("world")]; // slice of `String`
    assert!(v.iter().any(|e| e == "hello")); // search with `&str`
    assert!(!v.iter().any(|e| e == "hi"));

    let n = vec!["a", "b", "c", "d"];
    if n.iter().any(|&i| i == "c") {
        println!("Exists");
    }

    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
}
