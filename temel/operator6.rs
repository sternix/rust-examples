use std::cmp::PartialEq;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;

#[derive(Debug, Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;

    fn neg(self) -> Complex<O> {
        return Complex {
            re: -self.re,
            im: -self.im,
        };
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// bunun yerine #[derive(PartialEq)]
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == self.im
    }
}

impl fmt::Display for Complex<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RE:{} IM:{}", self.re, self.im)
    }
}

fn main() {
    let a = Complex { re: 5, im: 5 };
    let b = Complex { re: 10, im: 10 };
    println!("{}", a + b);

    let c = Complex { re: 6, im: 8 };
    println!("{}", -c);

    let mut d = Complex { re: 3, im: 3 };
    d += b;

    println!("{}", d);

    assert!(a != b);
}
