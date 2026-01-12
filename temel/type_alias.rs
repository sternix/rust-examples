// Type aliases: the type keyword can be used like typedef in C++, to declare a new name for an existing type.

type Table = std::collections::HashMap<String, Vec<String>>;

fn main() {
    let mut tbl = Table::new();
    tbl.insert("abc".into(), vec!["x".into(), "y".into()]);
}
