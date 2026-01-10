// Bir trait'i istersek tipin referansına tanımlayabiliriz

struct MyT;

// fakat sadece tipe tanımlarsak hem referanslar hem normal'i çalışıyor
impl std::fmt::Display for MyT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "MyT")
    }
}

struct MyS;

impl std::fmt::Display for &MyS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "MyS")
    }
}

struct MyM;

// yada &mut MyS ile sadece mutable referanslar için tanımlayabiliriz

impl std::fmt::Display for &mut MyM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "MyM")
    }
}

impl std::fmt::Display for Box<MyS> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "Boxed")
    }
}

fn main() {
    let mut t = MyT;
    println!("{}", t);
    println!("{}", &t);
    println!("{}", &mut t);

    let mut s = MyS;
    println!("{}", &s);

    // bunlar hata veriyor
    // the trait `std::fmt::Display` is not implemented for `MyS`

    // println!("{}", s);
    // println!("{}", &mut s);

    let mut m = MyM;
    println!("{}", &mut m);

    // bunlar hata veriyor
    // the trait `std::fmt::Display` is not implemented for `MyS`

    //println!("{}", m);
    // println!("{}", &m);

    println!("{}", Box::new(s));
}
