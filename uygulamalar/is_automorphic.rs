// https://twitter.com/Mastapegs/status/1508246331244138501?cxt=HHwWisC--YKDr-4pAAAA

// A number is called an "Automorphic number" if and only if its square ends in the same digits as the number itself.

/*

5 => 5 * 5 = 25, sonu 5 ile bitiyor
6 => 6 * 6 = 36, sonu 6 ile bitiyor
25 => 25 * 25 = 625, sonu 25 ile bitiyor

*/

fn is_automorphic(n: u64) -> bool {
    n.pow(2).to_string().ends_with(&n.to_string())
}

fn main() {
    for i in 1..1_000_000 {
        if is_automorphic(i) {
            println!("{i} is automorphic number");
        }
    }
}
