/*
A let declaration can declare a variable without initializing it. The variable can then be initialized with a later
assignment.

it's an error for use a variable before it's initialized.
*/

fn main() {
    let name;

    if true {
        name = "XXX";
    } else {
        name = "YYY";
    }

    println!("{}", name);
}
