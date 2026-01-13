// Replacing with a Function

use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace_all("123 456 789", |caps: &regex::Captures| {
        // eğer eşleşme bulunmazsa burası çalışmıyor
        let num: i32 = caps[0].parse().unwrap();
        (num * 2).to_string()
    });

    assert_eq!(result, "246 912 1578");
}

/*
sırasıyla
123
456
789
geliyor

2 ile çarpıp string olarak dönüyor, dönenler birleştiriliyor

123 * 2
456 * 2
789 * 2

*/
