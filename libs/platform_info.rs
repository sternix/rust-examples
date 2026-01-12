/*

[dependencies]
platform-info = "0.2"

*/

use platform_info::*;

fn main() {
    let uname = PlatformInfo::new().unwrap();
    println!("{}", uname.sysname());
    println!("{}", uname.nodename());
    println!("{}", uname.release());
    println!("{}", uname.version());
    println!("{}", uname.machine());
}
