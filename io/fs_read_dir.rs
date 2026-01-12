use std::fs;

fn main() {
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("{:?}", path.unwrap().path());
            }
        }
    }
}
