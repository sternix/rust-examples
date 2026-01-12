// Basit enum tipi

#[derive(Debug)]
enum Enum {
    Unit,
    Tuple(u32, char, String),
    Struct { x: u32, y: String },
}

impl Enum {
    // statik metod
    fn new() -> Self {
        Enum::Unit
    }

    // dinamik metod
    fn print(&self) {
        match self {
            Enum::Unit => println!("Unit Enum"),
            Enum::Tuple(u, c, s) => println!("Tuple Enum 0: {},1: {}, 2: {}", u, c, s),
            Enum::Struct { x, y } => println!("Struct Enum: x: {}, y: {}", x, y),
        }
    }

    fn debug(&self) {
        println!("DEBUG: {:?}", self);
    }
}

fn main() {
    let e = Enum::Unit;
    e.print();

    let e = Enum::Tuple(1, 'X', "tuple enum".to_string());
    e.print();

    let e = Enum::Struct {
        x: 2,
        y: "struct enum".to_string(),
    };
    e.print();
    e.debug();

    let e = Enum::new();
    e.print();

    let e = Enum::Struct {
        x: 3,
        y: "abc".to_string(),
    };

    if let Enum::Struct { x, y } = e {
        println!("x: {}, y: {}", x, y);
    }
}
