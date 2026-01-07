fn main() {
    for i in 0..5 {
        println!("{}", i);
    }
}

/*
burada 0..5 yapısına range deniliyor
0 dan 5 kadar (fakat 5 dahil değil) bir aralık tanımlanıyor
0..5 => 0,1,2,3,4 değerlerini döndürüyor

eğer 5'inde dahil olmasını istersek
0..=5 => 0,1,2,3,4,5 değerlerini döndürüyor

for i := 0; i < 10; i++ {} ifadesi
*/
