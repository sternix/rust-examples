/*
bazen işlemin başarısız olmasını ister yada bekleriz,
fonksiyonun hata vermesi test'in başarılı olduğu anlamına gelebilir
*/

#[test]
#[should_panic(expected = "divide by zero")]
fn test_xyz() {
    let a = 1;
    let b = 0;
    println!("{}", a / b);
}
