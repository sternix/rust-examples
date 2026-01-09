// If you want to pattern match on a boxed value, you may have to dereference the box manually.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let boxed_point = Box::new(Point { x: 0, y: 0 });

    match *boxed_point {
        // buraya * koymazsak hata veriyor
        Point { x, y } => println!("Point at x:{} , y:{}", x, y),
    }
}

// anlaşılan pattern match'de Deref yapılmıyor
