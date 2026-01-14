/*
if a type implements Drop, it cannot implement the Copy trait.
if a type is Copy, that means that simple byte-for-byte duplication is sufficient to produce an independent copy of that value.

The standard prelude includes a function to drop a value, drop, but its definition is anything but magical:

fn drop<T>(_x: T) {}

In other words, it receives its argument by value, taking ownership from the caller and then does nothing with it. Rust drops the value of _x when is goes out of scope, as it would for any other variable.

kısaca çağrılan değişkenin sahipliğini alıyor.
bu değişken daha kullanılamıyor. scope'u bitiyor.
*/

struct St {
    f: String,
    v: Vec<String>,
}

impl Drop for St {
    fn drop(&mut self) {
        print!("Dropping {}", self.f);
        if !self.v.is_empty() {
            print!(" (AKA {})", self.v.join(", "));
        }
        println!("");
    }
}

fn main() {
    let mut a = St {
        f: "A".to_string(),
        v: vec!["X".to_string(), "Y".to_string()],
    };

    println!("Before assignment");
    a = St {
        f: "B".to_string(),
        v: vec![],
    };

    println!("At end of block");
}

/*
eğer a.drop() yazsak derleyici

explicit destructor calls not allowed
şeklinde hata veriyor

*/
