// iterator max_by_key, min_by_key metodları

use std::collections::HashMap;

fn main() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"Portland", &583_776))
    );

    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Greenhorn", &2))
    );

    // yada name alanına göre sıralamak için
    match populations.iter().max_by_key(|&(n, _p)| n) {
        Some(n) => println!("name: {:?}", n),
        None => {}
    }

    println!("{:?}", populations.iter().min_by_key(|&(n, _p)| n).unwrap())
}
