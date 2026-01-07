// zip - fermuar

/*
ziplemek fermuar gibi iki tarafı ifade ediyor
karşılıklı iki eleman bir tuple oluşturuyor
her birinden birer tane alıp bir tuple yapılıyor
az elemanlı olan iter'in sayısı kadar tuple oluşturulur,
çok elemanlı olanın az elemanlı olan kadar elemanı kullanılır
*/

fn main() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    for i in a1.iter().zip(a2.iter()) {
        println!("({},{})", i.0, i.1);
    }
}

/*

(1,4)
(2,5)
(3,6)

*/
