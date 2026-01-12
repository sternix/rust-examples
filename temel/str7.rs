fn main() {
    let text = "the red fox and the lazy dog";

    let words: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", words);
    // ["the", "red", "fox", "and", "the", "lazy", "dog"]
    // yada

    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("{:?}", words);

    // yada String olarak
    let words: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();
    println!("{:?}", words);
}
