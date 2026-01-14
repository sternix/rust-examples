fn main() {
    let x: std::collections::HashSet<_> = {
        let unique = "hello".chars();
        unique.filter(|&char| char != 'h').collect()
    };

    println!("{:?}", x);
}
// {'e', 'l', 'o'}
