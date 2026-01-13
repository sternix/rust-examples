// Iterating Over Matches

use regex::Regex;

fn main() {
    let re = Regex::new(r"\b\w+\b").unwrap();
    for cap in re.captures_iter("hello world diğer kelime") {
        println!("{}", cap.get(0).unwrap().as_str());
    }
}

/*

cap.get(0) -> tüm kelime
cap.get(1) -> sadece eşleşen

*/
