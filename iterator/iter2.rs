/*
If there's a natural way to iterate over some type, it can implement
std::iter::IntoIterator, whose into_iter method takes a value and returns an
iterator over it:
*/

fn main() {
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{}", element);
    }

    // aynı işi yapıyor

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    // burada bize göstermeden (&v).into_iter() ile bir iterator oluşturuyor.
}
