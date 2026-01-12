fn main() {
    let iter = (0..10)
        .filter(|x| x % 2 == 0) // sadece 2'ye tam bölünebilenler
        .map(|x| x * x) // filter'dan dönenlerin karesini al
        .chain(10..12); // map'ten dönenlere 10..12 range'ini ekle 12 dahil değil

    // buraya kadar iter oluşmadı
    // sadece
    // Chain { a: Some(Map { iter: Filter { iter: 0..10 } }), b: Some(10..12) }
    // şeklinde bir struct oluşturuldu
    // println!("{:?}", iter);

    // şimdi consume edildi, next() metodu çağrıldı
    for i in iter {
        println!("{i}");
    }
}

/*

0
4
16
36
64
10
11

*/
