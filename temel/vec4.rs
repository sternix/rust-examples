fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop(); // son elemanı yani 40'ı çıkarıyor
    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    assert_eq!(v1, v2);
    v2.extend(0..2); // vectore kendi tipinden başka bir vectörü ekliyor.
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}
