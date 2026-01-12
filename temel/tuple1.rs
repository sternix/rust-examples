fn main() {
    // unit tipi, empty tuple, empty type
    // hem tip, hem tanımlama için kullanılabilir
    let _x: () = ();

    // parantez içindeki bir ifade
    // let x = 1; ile let x = (1); aynı olduğundan
    // "parantez gereksiz" uyarısı veriyor
    let x = (1);
    assert_eq!(x, 1);

    // virgül ile tuple olduğunu bildiriyoruz
    // bir elemanlı tuple
    let x = (1,);
    assert_eq!(x, (1,));

    // tip tanımlaması
    let _x: (i32, u32);

    // ilklendirme
    _x = (123, 456);
}
