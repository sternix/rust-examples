use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct X {
    a: i32,
}

fn cmpf(lhs: &&f64, rhs: &&f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn cmpx(lx: &&X, rx: &&X) -> Ordering {
    lx.a.cmp(&rx.a)
}

fn main() {
    let a = [-2.0, 0.5, 1.0, -5.3, 10.4];
    assert_eq!(a.iter().max_by(cmpf), Some(&10.4));
    assert_eq!(a.iter().min_by(cmpf), Some(&-5.3));

    let a = [
        X { a: 1 },
        X { a: -1 },
        X { a: 0 },
        X { a: 5 },
        X { a: 10 },
        X { a: -5 },
    ];

    assert_eq!(a.iter().max_by(cmpx), Some(&X { a: 10 }));
    assert_eq!(a.iter().min_by(cmpx), Some(&X { a: -5 }));
}
