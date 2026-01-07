struct Normal {
    id: i32,
    adi: String,
    soyadi: String,
}

impl Normal {
    // statik metod
    fn new(id: i32, adi: String, soyadi: String) -> Self {
        Normal { id, adi, soyadi }
    }

    // dinamik metod
    fn print(&self) {
        println!(
            "Normal struct alanları: id: {}, adi: {}, soyadi: {}",
            self.id, self.adi, self.soyadi
        );
    }
}

struct Tuple(i32, String, String);

impl Tuple {
    fn new(id: i32, adi: String, soyadi: String) -> Self {
        Tuple(id, adi, soyadi)
    }

    fn print(&self) {
        println!(
            "Tuple struct alanları: t.0: {}, t.1: {}, t.2: {}",
            self.0, self.1, self.2
        );
    }
}

struct Unit;

impl Unit {
    fn statik_method() {
        println!("statik metod çağrıldı");
    }

    fn dinamik_method(&self) {
        println!("dinamik metod çağrıldı");
    }
}

fn main() {
    let n = Normal {
        id: 1,
        adi: "adi".to_string(),
        soyadi: "soyadi".to_string(),
    };
    n.print();

    let n = Normal::new(2, "adi2".to_string(), "soyadi2".to_string());
    n.print();

    let t = Tuple(3, "adi3".to_string(), "soyadi3".to_string());
    t.print();

    let t = Tuple::new(4, "adi4".to_string(), "soyadi4".to_string());
    t.print();

    Unit::statik_method();

    let u = Unit;
    u.dinamik_method();
}
