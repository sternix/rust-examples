/*

iterator'un min ve max metodları

std::cmp::Ord traitini impl eden tiplerde kullanılabilir
f32 ve f64 Ord'u değil PartialOrd'u impl ettiğinden kullanılamıyor

*/

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct X {
    a: i32,
}

fn main() {
    let a = [-2, 0, 1, -5, 10];
    assert_eq!(a.iter().max(), Some(&10));
    assert_eq!(a.iter().min(), Some(&-5));

    let a = [
        X { a: 1 },
        X { a: -1 },
        X { a: 0 },
        X { a: 5 },
        X { a: 10 },
        X { a: -5 },
    ];

    assert_eq!(a.iter().max(), Some(&X { a: 10 }));
    assert_eq!(a.iter().min(), Some(&X { a: -5 }));
}
