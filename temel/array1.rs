/*
Sabit sayıda eleman - Ekleme veya çıkartma yapılamıyor
Tek tip - tüm elemanlar aynı tip'te
Tanımlanırken uzunluğu bildiriliyor


ekleme ve çıkartma yapılamayan ilk verilen uzunluk ile işlem yapılan dizi
eğer mut ise elemanları değişebiliyor.
*/

fn main() {
    let a1 = ["sıfır", "bir", "iki"];

    // [Tip, len]
    let a2: [&str; 3] = ["sıfır", "bir", "iki"];
    // dikkat edilirse tip verilmemiş, verilen değerden bu kadar
    let a3 = ["TEST"; 3]; // ["TEST","TEST","TEST"]
    let a4: [i32; 3] = [0, 1, 2];

    for v in a1.iter() {
        println!("{}", v);
    }

    for v in a2.iter() {
        println!("{}", v);
    }

    for v in a3.iter() {
        println!("{}", v);
    }

    for v in a4.iter() {
        println!("{}", v);
    }

    let mut a = [0, 1, 2];

    a[1] = 5;

    for v in a.iter() {
        println!("{}", v);
    }

    for v in &a {
        println!("{}", v);
    }

    for v in &a[..] {
        println!("{}", v);
    }
}

// array elemanlarını dönmek için iter(), &array yada &[..] şeklinde kullanabiliriz.
