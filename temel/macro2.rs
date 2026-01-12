#[macro_export]
macro_rules! hashmap {
    ($($( $key: expr => $val: expr )+$(,)?)*) => {{
         let mut map = ::std::collections::HashMap::new();
         $($( map.insert($key, $val); )*)*
         map
    }}
}

fn main() {
    let hm = hashmap!(
        'h' => 89,
        'a' => 1,
        's' => 19,
        'h' => 8,
    );

    println!("{:?}", hm);
}
