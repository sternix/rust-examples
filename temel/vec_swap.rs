fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.swap(1, 3);
    assert_eq!(v, vec![1, 4, 3, 2]);

    // index'teki elemanı çıkarıp yerine son elemanı koyuyor
    // normal remove işlemi çıkan elamandan sonraki tüm elemanları kaydırıyordu
    let rem = v.swap_remove(0);
    assert_eq!(rem, 1);
    assert_eq!(v, vec![2, 4, 3]);
}
