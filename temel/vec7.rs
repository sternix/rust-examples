fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("len: {}, cap: {} ", v.len(), v.capacity());
    v.push(10);
    println!("len: {}, cap: {} ", v.len(), v.capacity());

    let mut v = Vec::new(); // tip daha belli değil
    // ilk ekleyeceğimiz tipteki değer Vec'in tipi oluyor
    v.push("test");
    // v.push(10);
    // ^^ expected &str, found integer

    // buradaki 1024 dinamik olarak hesaplanan bir değer olabilir
    let v = vec![0; 1024];
    println!("len: {}, cap: {} ", v.len(), v.capacity());
    // 1024, 1024

    let mut v = Vec::with_capacity(100);
    v.push(10);
    println!("len: {}, cap: {} ", v.len(), v.capacity());

    // burada tipi bildirmeliyiz sonuçta i8 mi i32'mi bildirmeliyiz.
    let mut v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
    v.reverse();
    assert_eq!(v, [4, 3, 2, 1, 0]);

    v.insert(3, 30); // index sıfırdan başlıyor, yani 1'in yerine
    // eklenecek 1 ötelenerek 4. eleman olacak
    assert_eq!(v, [4, 3, 2, 30, 1, 0]);
    v.remove(3);
    assert_eq!(v, [4, 3, 2, 1, 0]);

    assert_eq!(v.pop(), Some(0));
    // pop sondaki elemanı vector'den açıkarıp sonuç olarak döndürüyor
    // sonucun ne olacağı belli olmadığından Option<T> olarak dönüyor
    v.pop(); // Some(1)
    v.pop(); // Some(2)
    v.pop(); // Some(3)
    v.pop(); // Some(4)
    assert_eq!(v.pop(), None);
}
