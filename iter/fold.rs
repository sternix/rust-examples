// The fold method is a very general tool for accumulating some sort of result over the entire sequence of items an iterator produces. Given an initial value, which we'll call the accumulator, and a closure, fold repeatedly applies the closure to the current accumulator and the next item from the iterator. The value the closure returns is taken as the new accumulator, to be passed to the closure with the next item. The final accumulator value is what fold itself returns. if the sequence is empty, fold simply returns the initial accumulator.

/*
fn fold<A,F>(self, init: A, f: F) -> A
    where Self: Sized, F: FnMut(A, Self::Item) -> A;
*/

fn main() {
    let a = [5, 6, 7, 8, 9, 10];

    // count
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);

    // sum
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);

    // product
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151_200);

    // max
    assert_eq!(
        a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m, i)),
        10
    );
    assert_eq!(a.iter().fold(a[0], |m, &i| std::cmp::max(m, i)), 10);

    // min
    assert_eq!(
        a.iter().fold(i32::max_value(), |m, &i| std::cmp::min(m, i)),
        5
    );
    assert_eq!(a.iter().fold(a[0], |m, &i| std::cmp::min(m, i)), 5);
}
