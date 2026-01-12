fn find(value: i32, slice: &[i32]) -> Option<usize> {
    for (index, &element) in slice.iter().enumerate() {
        if element == value {
            return Some(index);
        }
    }
    None
}

fn main() {
    let array = [1, 2, 3, 4, 5];

    if let Some(index) = find(2, &array) {
        println!("The element 2 is at index {}", index);
    }

    if let None = find(12, &array) {
        println!("The element 12 is not in array");
    }

    if find(12, &array).is_none() {
        println!("Tje element 12 is not in the array");
    }
}
