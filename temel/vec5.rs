fn main() {
    let mut vec = Vec::new();
    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());

    vec.push(1);
    vec.push(2);

    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());

    // ilk sıraya (0. index) 3 elemanını ekle
    vec.insert(0, 3);
    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());

    let removed = vec.remove(0);
    println!("removed : {}", removed);
    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());

    let clone = vec.clone();
    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());
    println!(
        "clone dizi:{:?} clone len:{} clone cap:{}",
        clone,
        clone.len(),
        clone.capacity()
    );

    vec.clear();
    println!("dizi:{:?} len:{} cap:{}", vec, vec.len(), vec.capacity());
}

/*
dizi:[] len:0 cap:0
dizi:[1, 2] len:2 cap:2
dizi:[3, 1, 2] len:3 cap:4
removed : 3
dizi:[1, 2] len:2 cap:4
dizi:[1, 2] len:2 cap:4
clone dizi:[1, 2] clone len:2 clone cap:2
dizi:[] len:0 cap:4
*/
