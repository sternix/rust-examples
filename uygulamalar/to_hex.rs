// https://twitter.com/Mastapegs/status/1508264308974788612/photo/1

macro_rules! compare {
    ( $got: expr, $expected : expr) => {
        if $got != $expected {
            panic!("\nGot: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

fn to_hex(n: i32) -> String {
    // eğer 2 haneden küçükse başına 0 koyuyor 15 -> 0F gibi
    format!("{:0>2X}", n)
}

fn main() {
    compare!(to_hex(15), "0F");
    compare!(to_hex(255), "FF");
}
