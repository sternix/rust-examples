// See how adding parantheses, changed the result. It is not about the order here, always && will happen before ||

fn main() {
    let (a, b, c) = (true, true, false);
    let mut d;

    println!("a is {}, b is {}, c is {}", a, b, c);

    d = a || b && c;
    println!("a || b && c, \t => {}", d);

    d = (a || b) && c;
    println!("(a || b) && c, \t => {}", d);

    d = c && a || b;
    println!("c && a || b, \t => {}", d);
}

/*

a is true, b is true, c is false
a || b && c,     => true
(a || b) && c,   => false
c && a || b,     => true

*/
