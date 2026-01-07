// bir trait'te illa metod olmasına gerek yok
trait T1 {}

struct S1;

impl T1 for S1 {}

// sadece T1 traitini impl edenler
fn t1<T: T1>(_t: &T) {}

trait T2: T1 {}

struct S2;

impl T1 for S2 {}
impl T2 for S2 {}

fn t2<T: T2>(_t: &T) {}

trait T3 {
    fn st(); // trait'in statik metodu
}

struct S3;
struct S3_2;

impl T3 for S3 {
    fn st() {
        println!("T3::S3");
    }
}

impl T3 for S3_2 {
    fn st() {
        println!("T3::S3_2");
    }
}

fn t3<T: T3>(_t: T) {
    T::st()
}

trait T4 {
    fn sd() {
        println!("static default method");
    }

    fn d(&self) {
        println!("instance default method");
    }
}

struct S4;

impl T4 for S4 {}

struct S4_2;

impl T4 for S4_2 {
    fn sd() {
        println!("Overrided default static method of S4_2");
    }
}

struct S4_3;

impl T4 for S4_3 {
    fn d(&self) {
        println!("overrided default instance method for S4_3");
    }
}

fn t4<T: T4>(t: T) {
    T::sd(); // static
    t.d(); // instance
}

fn main() {
    let s = S1;

    t1(&s);

    let s2 = S2;
    t1(&s2);
    t2(&s2);

    S3::st();
    S3_2::st();

    t3(S3);
    t3(S3_2);

    t4(S4);
    t4(S4_2);
    t4(S4_3);
}
