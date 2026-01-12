/*

todo! ve unimplemented!
ikiside panicliyor

*/

fn main() {
    todo1();
    unimpl1();
    todo2();
    unimpl2();
}

// not implemented
fn unimpl1() {
    unimplemented!();
}

// not implemented: Custom Message
fn unimpl2() {
    unimplemented!("Custom Message");
}

// not yet implemented
fn todo1() {
    todo!();
}

// not yet implemented: Custom Message
fn todo2() {
    todo!("Custom Message");
}
