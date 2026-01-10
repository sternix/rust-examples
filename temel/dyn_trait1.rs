trait Shape {
    fn area(&self) -> i32;
}

struct Square;
struct Triangle;
struct Circle;
struct Rectangle;

impl Shape for Square {
    fn area(&self) -> i32 {
        1
    }
}

impl Shape for Triangle {
    fn area(&self) -> i32 {
        2
    }
}

impl Shape for Circle {
    fn area(&self) -> i32 {
        3
    }
}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        4
    }
}

// burada farklı tipler döndüğü için
// impl Trait olmuyor
fn get_shape(i: i32) -> Box<dyn Shape> {
    match i {
        1 => Box::new(Square),
        2 => Box::new(Triangle),
        3 => Box::new(Circle),
        _ => Box::new(Rectangle),
    }
}

fn main() {
    for i in 1..5 {
        let s = get_shape(i);
        println!("{}", s.area());
    }
}
