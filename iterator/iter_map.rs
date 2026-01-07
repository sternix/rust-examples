/*
map : Takes a closure and creates an iterator which calls that closure on each element.

parametre olarak bir closure yada fonksiyon alıp,
bir iterator oluşturuyor,
çağrıldığında bu iterator'deki herbir elemanı fonksiyona parametre olarak verip fonksiyonu çalıştırıyor

map verilen iterator'ün elemanlarını verilen closure'a yollayıp dönen değerlerden başka bir iterator oluşturuyor,

map'in kullanım alanlarından biri A tipindeki iterator'ü B tipine dönüştürebilmek.
aşağıdaki örnekte sayı iteratör'ü char iterator'üne dönüştürebildik.

map olmadan orj iterator'ü alıyoruz, bir vector oluşturuyoruz,
orjinal iteratör'ün her bir elemanını fonksiyona yollayıp dönen değeri yeni oluşturduğumuz
vector'e ekliyoruz. bu işlemi map ile çok kısa bir şeilde yapabiliriz.
*/

fn main() {
    let a = [1, 2, 3, 4];

    for i in a.iter().map(|x| x * x) {
        println!("{}", i);
    }
}

// burada for döngüsünde map ile oluşturulmuş veri yapısını kullanıyoruz (consume)
