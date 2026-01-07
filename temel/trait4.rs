#[derive(Debug)]
struct Cat {
    name: String,
}

impl Cat {
    fn miyav(&self) {
        println!("{}", self.name);
    }
}

#[derive(Debug)]
struct Dog {
    typ: String,
}

impl Dog {
    fn hav(&self) {
        println!("{}", self.typ);
    }
}

// where ifadesi olmadan tip'ten sonra sınırlama
fn test2<T: std::fmt::Debug>(t: T) -> i32 {
    println!("{:?}", t);
    28
}

// where ifadesi ile
fn test<T>(t: &T) -> i32
where
    T: std::fmt::Debug,
{
    println!("{:?}", t);
    28
}

fn main() {
    let cat = Cat {
        name: String::from("dobi"),
    };

    test(&cat);
    cat.miyav();
    test2(&cat);
    test2(cat);

    let dog = Dog {
        typ: String::from("kangal"),
    };

    // test için mutlaka tipin referansını yollamalıyız.
    test(&dog);
    dog.hav();
    // test2 için hem referans hem değeri yollayabiliriz
    test2(&dog);
    test2(dog);
}
