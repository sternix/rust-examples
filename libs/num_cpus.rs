/*

[dependencies]
num_cpus = "1"

*/

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
