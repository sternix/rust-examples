// ; ifadesi empty statement olarak kullanılabilir

use std::thread::sleep;
use std::time::Duration;

fn work() {
    println!("Working");
    sleep(Duration::from_millis(1000));
}

fn sleeping() {
    println!("Sleeping");
    sleep(Duration::from_millis(1000));
}

fn main() {
    loop {
        work();
        sleeping();
        /* rustfmt ile siliniyor
              warning: unnecessary trailing semicolon
        --> empty_statement.rs:20:9
         |         ;
         |         ^ help: remove this semicolon
              ;
              */
    }
}
