/*

Tuple = Tanımlama Grubu

tuples: Fixed size ordered list of elements of different(or same) data types

yeni bir eleman ekleme yada çıkartma yapılamıyor,
eğer mut ise mevcut elemanlar ilk verilen tip ile aynı olmak şartıyla değiştirilebilir.

*/

fn main() {
    let a = (1, 1.5, true, 'a', "Hello, world!");
    // a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello, world!"

    // dikkat edilirse a.0 vs şeklinde erişebiliyoruz

    let b: (i32, f64) = (1, 1.5);

    let (c, d) = b; // c = 1, d = 1.5
    let (e, _, _, _, f) = a; //e = 1, f = "Hello, world!", _ indicates not interested of that item

    let g = (0,); //single-element tuple
    // eğer virgülü koymazsak int değişkeni olarak değiştirin diye uyarıyor

    let h = (b, (2, 4), 5); //((1, 1.5), (2, 4), 5)

    // içiçe elemanlara erişirken
    println!("{}", (h.1).0); // 2

    println!("{:?}", a); //(1, 1.5, true, 'a', "Hello, world!")
}
