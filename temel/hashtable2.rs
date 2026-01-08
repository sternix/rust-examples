use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("x".to_string(), vec!["b".to_string(), "a".to_string()]);
    table.insert("y".to_string(), vec!["d".to_string(), "c".to_string()]);
    table.insert("z".to_string(), vec!["f".to_string(), "e".to_string()]);

    show(&table);
    sort(&mut table);
    show(&table);
}

fn show(table: &Table) {
    for (k, v) in table {
        println!("key: {}", k);
        for i in v {
            println!("\t{}", i);
        }
    }
}

// dikkat edilirse v yani vector'de mut olarak yollandı
fn sort(table: &mut Table) {
    for (_, v) in table {
        v.sort();
    }
}
