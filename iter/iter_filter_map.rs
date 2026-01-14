fn main() {
    let maybe_numbers = vec!["1", "2", "nah", "nope", "3"];
    let numbers: Vec<_> = maybe_numbers
        .into_iter()
        .map(|i| i.parse::<u64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();
    println!("{:?}", numbers);

    // yerine

    let maybe_numbers = vec!["1", "2", "nah", "nope", "3"];
    let numbers: Vec<_> = maybe_numbers
        .into_iter()
        .filter_map(|i| i.parse::<u64>().ok())
        .collect();
    println!("{:?}", numbers);
}

/*

parse returns a Result, but filter_map expects an Option.
We can convert a Result into an Option by calling ok() on it

The filter_map is similar to the select method in Ruby:

[1, 2, 3, 4, 5].select { |element| element.even? }

*/
