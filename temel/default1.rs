struct Foo {
    id: u32,
    adi: String,
    soyadi: String,
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            id: 0,
            adi: String::new(),
            soyadi: String::new(),
        }
    }
}

impl Foo {
    fn new() -> Foo {
        Foo {
            ..Default::default()
        }
    }

    fn to_str(&self) -> String {
        format!("{}-{}-{}", self.id, self.adi, self.soyadi)
    }
}

fn main() {
    let f = Foo::new();
    //let f = Foo{..Default::default()};
    println!("{}", f.to_str())
}
