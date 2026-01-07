/*
eğer bir fonksiyonun son ifadesi
return XXX;
şeklinde ise kısaca
XXX
yazabiliriz. dikkat edilirse XXX'ten sonra ; karakteri yok
*/

/*
This is because the body of the function (inside {}) has the value of its last expression,
just like with if-as-an-expression.

It's not wrong to use return, but code is cleaner without it.
You will still use return for returning early from a function.
*/

// burada sqr fonksiyonunu

fn sqr1(x: f64) -> f64 {
    return x * x;
}

// yerine

fn sqr2(x: f64) -> f64 {
    x * x
}

fn abs(x: f64) -> f64 {
    if x > 0.0 { x } else { -x }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial1(n: u64) -> u64 {
    if n == 0 {
        return 1;
    } else {
        // eğer parantez koyarsak warning veriyor
        // return (n * factorial1(n - 1))
        return n * factorial1(n - 1);
    }
}

fn factorial2(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial2(n - 1) }
}

fn main() {
    let res = sqr1(2.0);
    println!("square is {}", res);

    let res = sqr2(2.0);
    println!("square is {}", res);

    println!("abs: {}", abs(-6.0));
    println!("clamp: {}", clamp(3.0, 5.0, 10.0));

    println!("{}", factorial1(10));
    println!("{}", factorial2(10));
}
