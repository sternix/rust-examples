fn main() {
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
}

/*
flat_map ile
0 - 0..0
1 - 100.110
2 - 200..220
3 - 300..330
4 - 400..440

range'leri elde ediliyor
sonra bu range'ler tek bir range olarak dönüyor

map() değer dönen closure'larda işe yarıyor, dönen değerleri tek bir iterator olarak döndürüyor,

eğer closure iterator döndürürse her bir eleman için bir iterator döner
flat_map() ile dönen iterator'ler birleştirilip tek bir iterator olarak dönüyor.

enumerate her bir elemana bir index verip tuple olarak gönderiyor
100 geliyorsa
(1,100) olarak dönüyor

filter() ile verilen iteratorun elemanlarını bool döndüren bir fonksiyona gönderip
doğr

*/
