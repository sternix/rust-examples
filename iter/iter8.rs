fn main() {
    let my_map = (1..6).map(|x| x * x);
    println!("{:?}", my_map);
    // buraya kadar iterator kullanılmadı
    // sadece Map ... tipinde bir veri yapısı yapılandırıldı

    let my_map: Vec<_> = my_map.collect();
    // şimdi iterator tüm elemanları next() ile işledi

    //  yukarıdaki ile aynı
    //  let my_map = my_map.collect::<Vec<_>>();
    println!("{:?}", my_map);
}
