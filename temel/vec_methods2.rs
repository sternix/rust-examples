fn print_vec_info(v: &Vec<i32>) {
    println!("Len: {}", v.len());
    println!("Capacity: {}", v.capacity());
}

fn main() {
    let v = Vec::with_capacity(100);

    // Len: 0
    // Capacity: 100
    print_vec_info(&v);

    let mut v = vec![1, 2, 3, 4];

    // Len: 4
    // Capacity: 4
    print_vec_info(&v);

    v.reserve(100);
    // Len: 4
    // Capacity: 104
    print_vec_info(&v);

    // v.truncate(n) n'ninci eleman dahil sona doğru tüm elemanları siler
    v.truncate(1);
    // Len: 1
    // Capacity: 104
    print_vec_info(&v);

    // tüm elemanları siler, truncate(0) ile aynı
    v.clear();
    // Len: 0
    // Capacity: 104
    print_vec_info(&v);
}
