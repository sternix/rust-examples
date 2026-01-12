/*

A reference to an option &Option<T> cannot be unwrapped if the type T is not copyable.
The solution is to change the option to &Option<&T> using as_ref().

Converts from &Option<T> to Option<&T>.

*/

#[derive(Debug)]
struct Foo;

fn main() {
    let wrapped = Some(Foo);
    let wrapped_ref = &wrapped;

    println!("{:?}", wrapped_ref.as_ref().unwrap());
}

// burada as_ref'i kullanmazsak hata veriyor
