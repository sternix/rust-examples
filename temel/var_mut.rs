// https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1

fn do_numbery_things(mut x: &i32, y: &mut i32, mut z: &mut i32) {
    // This compiles, but doesn't mutate the value of `x` in
    // the calling context, because it's a mutable binding to
    // an immutable reference, meaning we can change what
    // it's bound to (binding it to `&6` here), but not the
    // value of what it's bound to.
    x = &6;

    // For that reason, this wouldn't compile, as it attempts
    // to mutate a value that is behind an immutable reference.
    //
    // *x = 6;

    // This works, because `y` is an immutable binding to a
    // mutable reference, so you can freely mutate the
    // referenced value so long as Rust's aliasing XOR
    // mutability rule isn't violated.
    *y = 12;

    // This wouldn't compile, however, because `y` is an
    // immutable binding, so you can't change what it's
    // bound to.
    //
    // y = 0;
    // This works, because `z` is a mutable reference, so same
    // as `y`, you can mutate the value it's referencing.
    *z = 9;

    // However, this _doesn't_ work, but for a distinct reason.
    // `&mut 8` is a temporary value which stops existing when
    // the function ends, and for a mutable reference (unlike
    // an immutable reference), the value needs to exist past the
    // end of the function, in the calling context. That's not
    // possible for a temporary value, so this would fail to
    // compile.
    //
    // z = &mut 8;
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;

    do_numbery_things(&x, &mut y, &mut z);

    println!("({}, {}, {})", x, y, z);

    // Prints "(1, 12, 9)" showing that `y` and `z`
    // were mutated by the function.
}
