fn main() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));

    // bitince başa dönüyor

    use std::iter::{once, repeat};

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }

    // 3'e bölünenler fizz
    // 5'e bölünenler buzz
    // 3 ve 5'e tam bölünenler fizzbuzz
    // dikkat edilecek kısım take(2) -> 3, take(4) -> 5'e denk geliyor
    // bölme kontrolü yapmıyor
}
