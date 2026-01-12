// closure'lar aslında bir fonksiyon

fn main() {
    let y = |x| x * 2;

    let z: fn(i32) -> i32 = y;

    println!("{}", z(13));

    // eğer parametrelere tip verilmezse varsayılan olarak i32 tanımlanıyor
    // eğer farklı bir tip vermek istersek tipini belirtmemiz gerekiyor

    let y = |x: f32| x * 2.0;

    let z: fn(f32) -> f32 = y;

    println!("{}", z(13.2));

    // normal bir fonkisyonu da değişkene atayabiliyoruz
    fn plus_one(a: i32) -> i32 {
        a + 1
    }

    //burada b bir function pointer oluyor
    let b: fn(i32) -> i32 = plus_one;
    let c = b(5); //6
    println!("{c}");
}
