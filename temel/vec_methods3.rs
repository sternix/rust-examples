fn main() {
    let mut v = vec![1, 2, 3, 4];
    let a = [5, 6, 7, 8];

    // bir iterator'un elemanlarını vec'in sonuna ekliyor
    // çoklu push
    v.extend(&a);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8]);

    // truncate ile aynı fakat çıkarılan elemanları
    // bir vec halinde döndürüyor
    // çoklu pop
    let mut removed = v.split_off(4);
    assert_eq!(removed, vec![5, 6, 7, 8]);
    assert_eq!(v, vec![1, 2, 3, 4]);

    // removed'ın tüm elemanları v'ye move edildi
    // removed boş olarak kaldı
    // extend'den farkı extend iterator'a karışmıyor
    // iterator'un elemanları vec'e kopyalanıyor
    // burada taşınıyor
    v.append(&mut removed);
    assert_eq!(removed, vec![]);
    assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8]);

    let mut x: Vec<i32> = vec![];

    // verilen range'den elemanları silip, silinen elemanları bir iterator'da döndürüyor
    // Removes the specified range from the string in bulk, returning all removed characters as an iterator.
    x.extend(v.drain(4..));
    assert_eq!(x, vec![5, 6, 7, 8]);
}
