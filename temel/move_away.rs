fn move_away(_: String) { // /dev/null gibi
}

fn main() {
    let [john, roa] = ["John".to_string(), "Roa".to_string()];
    move_away(john);
    move_away(roa);

    // daha kullanamayız
    //println!("{john}"); 
}
