/*

[dependencies]
log = "0.4"
env_logger = "0.8"

*/

fn main() {
    env_logger::init();
    log_me("Hello Log...")
}

fn log_me(l: &str) {
    log::debug!("Log: {}", l);
}

/*

bu şekilde çalıştırdığımızda hiç bir şey görülmez fakat
setenv RUST_LOG debug
ile varsayılan error olan level'ı debug yaptığımızda

*/
