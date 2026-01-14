// https://dtolnay.github.io/rust-quiz/37

struct Drop0;

impl Drop for Drop0 {
    fn drop(&mut self) {
        print!("0");
    }
}

fn main() {
    {
        let _ = &Drop0;
        print!("1");
    }
    {
        _ = &Drop0;
        print!("1");
    }
}

/*

When Drop0 is created, in both cases, it's assigned to a temporary memory location. Temporary memory locations are normally scoped to the statement where they're created, so the value is dropped at the end of the statement unless it's moved to a new location.

In the assignment statement, _ = &Drop0, the underscore expression _, is used to ignore the binding, therefore not moving the value, and it ends up being dropped at the end of the statement. So, 0 is printed first then 1.

In contrast, with let statements, temporary lifetime extension can take place and extend the lifetime of the temporary until the end of the block containing the let. For Drop0, in the statement let _ = &Drop0, this happens because it's the operand of a borrow expression. So even if the value itself doesn't move, the lifetime of the temporary is extended until the end of the block and so 1 is printed first then 0.

*/
