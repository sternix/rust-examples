// Accessing Elements

fn main() {
    let lines = vec![
        "ilk".to_string(),
        "ikinci".to_string(),
        "ucuncu".to_string(),
    ];
    let numbers = vec![1, 2, 3, 4, 5];
    let buffer: Vec<u8> = (0..15).collect();

    // Get a reference to an element
    let _first_line = &lines[0];

    // Get a copy of an element
    let _fifth_number = numbers[4]; // Requires Copy
    let _second_line = lines[1].clone(); // Requires Clone
    // String yerine &str olsaydı gerek yoktu

    // Get a reference to a slice
    let _my_ref = &buffer[4..12];

    // Get a copy of a slice
    let _my_copy = buffer[4..12].to_vec(); // Requires Clone

    // All of these forms panic if an index is out of bounds
    // Rust Vec, Array, Slice indekslerinde usize kullanıyor
}
