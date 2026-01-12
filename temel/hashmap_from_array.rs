// HashMap array'den oluşturulabiliyor

use std::collections::HashMap;

fn main() {
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    println!("{:?}", solar_distance);
}
