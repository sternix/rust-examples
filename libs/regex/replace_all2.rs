/*

Replacing Matches

içinde Java geçenleri Rust ile değiştir
Java bulamazsa bir değişiklik yapmıyor
bulduğu tüm Java'ları Rust yapıyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"\bJava\b").unwrap();
    let result = re.replace_all("I love Java!", "Rust");
    assert_eq!(result, "I love Rust!");

    let re = Regex::new(r"\bJava\b").unwrap();
    let result = re.replace_all("I love Java! to Much Java every Java", "Rust");
    //println!("{}", result);
    assert_eq!(result, "I love Rust! to Much Rust every Rust");

    /*

    case insensitive hali
    içinde java,Java yada JAVA geçen tüm kelimeleri Rust ile değiştir

    */

    let re = Regex::new(r"\b(?i)Java\b").unwrap();
    let result = re.replace_all("I love Java!", "Rust");
    assert_eq!(result, "I love Rust!");
    let result = re.replace_all("I love JAVA!", "Rust");
    assert_eq!(result, "I love Rust!");
    let result = re.replace_all("I love java!", "Rust");
    assert_eq!(result, "I love Rust!");

    /*

    (?i)\bJava\b
    \b(?i)Java\b

    ikiside çalışıyor

    */

    let re = Regex::new(r"(?i)\bJava\b").unwrap();
    let result = re.replace_all("I love Java! to Much JAVA every java", "Rust");
    //println!("{}", result);
    assert_eq!(result, "I love Rust! to Much Rust every Rust");
}
