/*

fn exit(code: i32) -> !

the ! means that exit() never returns. it's a divergent function.

*/

fn endless() -> ! {
    loop {}
}

/*
içinde break olduğundan dolayı derlenmiyor
expected `!`, found `()`

Of course Rust then considers it an error if the function can return normally.

fn can_return() -> ! {
    let mut i = 0;
    loop {
        i += 1;
        if i == 1000 {
            break;
        }
    }
}
*/

fn main() {
    // can_return();
    endless();
}
