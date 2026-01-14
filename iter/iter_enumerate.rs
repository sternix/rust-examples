fn main() {
    let v = vec![1, 2, 3, 4];

    for (index, val) in v.iter().enumerate() {
        println!("Index: {}, Value: {}", index, val);
    }
}
