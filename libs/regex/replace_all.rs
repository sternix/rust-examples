/*

\n \t yada boşlukları (birden çok olsada) hepsini " " ile değiştirmek
replace_all ile değiştiriliyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap(); // \s means whitespace
    let replaced = re.replace_all("Hello  world!\nHow  are\tyou?", " ");
    assert_eq!(replaced, "Hello world! How are you?");
}
