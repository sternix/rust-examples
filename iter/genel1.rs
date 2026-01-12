use std::collections::HashSet;

fn main() {
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (power_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);

    assert_eq!(power_of_two.len(), 3); // 4,16,64
    assert_eq!(impure.len(), 4); // 9,25,36,49

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());

    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");
}
