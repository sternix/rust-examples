/*
as_slice ile &v aynı işi yapıyor
burada ilgiç olan slice'ın tipi u32 yada i32
yapabiliriz fakat u32 yaptığımızda vektör'e -3 gibi bir değer verdiğimizde
derleme hatası oluyor
*/

fn prt(s: &[u32]) {
    for i in s {
        println!("{}", i);
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    prt(v.as_slice());
    prt(&v);
}
