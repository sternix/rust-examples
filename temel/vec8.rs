fn main() {
    let langs: Vec<String> = std::env::args().skip(1).collect();

    for lang in langs {
        println!(
            "{}: {}",
            lang,
            if lang.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
