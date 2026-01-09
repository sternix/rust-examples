/*

Renaming
Another feature of declaring and assigning to new variables in a function signature pattern is that you can create a variable with a name different from the name of the field in the relevant type.

*/

struct Something {
    field_1: i32,
    field_2: f64,
}

// The `field_1: x` and `field_2: y` parts are assigning
// the values of `field_1` to `x` and `field_2` to `y`.
fn func(
    Something {
        field_1: x,
        field_2: y,
    }: Something,
) {
    println!("x: {}, y: {}", x, y);
}

fn main() {
    let x = Something {
        field_1: 5,
        field_2: 1.0,
    };

    func(x);
}
