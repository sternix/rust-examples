/*

Splitting a String
tüm whitespace'i trim edip split ediyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"\s+").unwrap();
    let parts: Vec<_> = re.split("split         on       whitespace").collect();
    assert_eq!(parts, vec!["split", "on", "whitespace"]);
}
