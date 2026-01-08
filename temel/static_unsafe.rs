static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        // shared reference to mutable static
        // hatası veriyor
        println!("BEFORE: {}", STASH);
        STASH = p;
        println!("AFTER: {}", STASH);
    }
}

fn main() {
    f(&1234);
}


/*

fn f(p: &i32) {}

fonksiyonu aslında

fn <'a>(p: &'a i32) {}

fonksiyonun pratik yazılışı ikiside aynı

burada 'a ifadesine "tick a" yani f fonksiyonunun lifetime parametresi deniliyor

you can read <'a> as "for any lifetime" 'a "so when we write fn f<'a>(p: &'a i32), we're defining
a function that takes a reference to an i32 with any given lifetime 'a
Since STASH lives for the program's entire execution, the reference type it holds must have a lifetime of the some length: Rust calls this the 'static lifetime

if we see a function with a signature like g(p: &i32) or with the lifetimes written out,
g<'a>(p: &'a i32) we can tell that it does not stash(saklamak) its argument p anywhere that will outlive the call.
There's no need to look into g's definition; the signature alone tells us what g can and can't do with its argument.

You only need to worry about lifetime parameters when defining functions and types; when using them, Rust infers the lifetimes for you.

*/