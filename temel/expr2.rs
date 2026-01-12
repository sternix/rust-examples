fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{} even", i)
        } else {
            println!("{} odd", i)
        }
    }

    /*
    rust'da if bir expression'dır yani if bloğunun bir sonucu olabilir
    bunu kullanarak az önceki örneği

    şeklinde yazabiliriz. dikkat edilirse if ifadesinin sonucu string bir değerdir
    tüm blokların string döndürmesi gerekiyor.

    bir de if bloğuna dikkat edilirse "even" ve "odd"'dan sonra ; konulmamış
    eğer konulursa derleyici hata verir.
    */

    for i in 0..5 {
        println!("{} {}", i, if i % 2 == 0 { "even" } else { "odd" })
    }
}
