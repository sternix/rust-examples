use std::collections::HashMap;

// type alias
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("x".to_string(), vec!["a".to_string(), "b".to_string()]);
    table.insert("y".to_string(), vec!["c".to_string(), "d".to_string()]);
    table.insert("z".to_string(), vec!["e".to_string(), "f".to_string()]);

    show(&table);

    println!("{}", table.len());
}

/*
Burada show fonksiyonuna shared referans yolladığımızda yani
yani show(&table) şeklinde tanımladığımızda
k tipi &String, v tipi &Vec<String> oluyor

eğer show(table) şeklinde olsaydı
k tipi String v tipi Vec<String> olacaktı ve table'ın sahipliğini show fonksiyonu alacaktı.
*/

fn show(table: &Table) {
    for (k, v) in table {
        println!("key: {}", k);
        for i in v {
            println!("\t{}", i);
        }
    }
}
