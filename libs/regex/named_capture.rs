/*

buradaki let Some(caps) ifadesi güzel
burada middle named capture olarak adlandırılıyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"Homer (?<middle>.)\. Simpson").unwrap();
    let hay = "Homer J. Simpson";
    let Some(caps) = re.captures(hay) else { return };
    assert_eq!("J", &caps["middle"]);
}
