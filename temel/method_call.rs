/*
A method is just a special kind of a function
These two calls are equivalent

"hello".to_string()
str::to_string("hello")
*/

fn main() {
    println!("{}", "hello".to_string());
    println!("{}", str::to_string("hello"));
    println!("{}", ToString::to_string("hello"));
    println!("{}", <str as ToString>::to_string("hello"));
}

/*

All four of these method calls do exactly the same thing.
Most often, you'll just write value.method(). The other forms are qualified method calls.
They specify the type or trait that a method is associated with.
The last form, with the angle brackets, specifies bot: a full qualified method call.

*/
