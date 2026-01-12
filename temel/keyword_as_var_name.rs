fn r#return() -> u8 {
    println!("Here is your number.");
    8
}

fn main() {
    let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    println!("{}", r#mut);

    r#mut = 12;

    println!("{}", r#mut);
    println!("{}", r#let);

    let my_number = r#return();
    println!("{}", my_number);
}

// keyword'ları değişken olarak kullanmak için
