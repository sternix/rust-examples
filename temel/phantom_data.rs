// Shared behavior

use std::marker::PhantomData;

trait Value {
    fn value() -> i32;
}

struct Magic<T> {
    phantom: PhantomData<T>,
}

struct A;
struct B;

impl Value for Magic<A> {
    fn value() -> i32 {
        5
    }
}

impl Value for Magic<B> {
    fn value() -> i32 {
        10
    }
}

impl<T> Magic<T>
where
    Self: Value,
{
    fn extra() -> i32 {
        Magic::<T>::value() + 5
    }
}

fn main() {
    println!("MA: {}, MB: {}", Magic::<A>::extra(), Magic::<B>::extra());
}
