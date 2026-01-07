// In the past, you had to explicitly call .iter() to iterate over an array. Now, arrays in Rust implement IntoIterator directly, for example:

fn main() {
    let cities = ["Toronto", "New York", "Melbourne"];

    for city in cities {
        print!("{}, ", city);
    }
    println!();
    // output: Toronto, New York, Melbourne,

    /*

    It's important to understand what is happening under the hood here. When you iterate over the array directly, the array’s into_iter() method is called. If the elements implement the Copy trait, they are copied into the loop variable. In the example above, each city is a string slice (&str), which implements Copy, so during iteration, each city is copied into the variable city. This is safe and efficient because string slices are lightweight and implement the Copy trait. The array will still be accessible after the iteration, because its elements are copied, not moved. For types that don’t implement Copy, such as String, the Rust complier doesn't have a choice but to move the elements out of the array during iteration, meaning you lose ownership in the original array. In other words, direct iteration over non-Copy types, consumes the array. The array becomes unusable after the iteration.

    Sometimes you might want to avoid copying or moving the elements altogether, especially if the elements are larger or if they do not implement Copy. In that case, you iterate over references to the array elements. You can do this in one of two ways:

    */

    // 1. Using .iter()

    let cities = [
        String::from("Toronto"),
        String::from("New York"),
        String::from("Melbourne"),
    ];

    for city in cities.iter() {
        // Here, `city` is a reference to a String (&String), so the values aren’t moved.
        print!("{}, ", city);
    }
    println!();
    // output: Toronto, New York, Melbourne,

    // 2. Using the & reference operator

    let cities = [
        String::from("Toronto"),
        String::from("New York"),
        String::from("Melbourne"),
    ];

    for city in &cities {
        // This is equivalent to calling cities.iter().
        print!("{}, ", city);
    }
    println!();
    // output: Toronto, New York, Melbourne,
}
