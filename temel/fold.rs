/*
fold: katlamak, kıvrım
This operation is sometimes called 'reduce' or 'inject'.
*/

fn main() {
    let v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    /*
       işlem şöyle gerçekleşiyor
       a'nın yani accumulator'ün ilk değeri fold'un ilk parametresi olan 1
       b'nin ilk değeri v.iter()'in birinci elemanı olan 2
       a = 1 * 2
       a 2, b nin değeri ikinci eleman olan 3 oldu
       a = 2 * 3
       a 6, b nin değeri üçüncü eleman olan 5 oldu
       a = 6 * 5
       a 30, b nin değeri dördüncü eleman olan 7 oldu
       a = 30 * 7
       iterator'de başka eleman kalmadığından sonuç accumulator olan a'nın değeri yani 210
    */
}
