/*
If there's a natural way to iterate over some type, it can implement
std::iter::IntoIterator, whose into_iter method takes a value and returns an
iterator over it:
*/

fn main() {
    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{element}");
    }

    let mut it = v.iter();
    while let Some(element) = it.next() {
        println!("{element}");
    }

    // iter() referans
    // into_iter() value alıyor

    // yukarıda v.into_iter() yapılsa bu satır hata veriyor
    println!("v yaşıyor, len: {}", v.len());
}
