use std::fs;
use std::io;

fn check(r: &io::Result<()>) {
    match r {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn print_metadata(m: &fs::Metadata) {
    // fs::FileType
    let ft = m.file_type();

    // bu üçü metadata'da var
    if ft.is_dir() {
        println!("Directory");
    } else if ft.is_file() {
        println!("File");
    } else if ft.is_symlink() {
        println!("Symlink");
    }

    println!("Len: {}", m.len());

    // Result<SystemTime>
    println!("Modified Time: {:?}", m.modified().unwrap());
    // Result<SystemTime>
    println!("Access Time: {:?}", m.accessed().unwrap());
    // Result<SystemTime>
    println!("Create Time: {:?}", m.created().unwrap());

    // m.permission()
    // std::os::unix::fs::PermissionsExt
}

fn main() {
    // mkdir
    let e = fs::create_dir("xxx");
    check(&e);

    // mkdir -p
    let e = fs::create_dir_all("xxx/abc/klmn");
    check(&e);

    // realpath
    match fs::canonicalize("xxx/abc/klmn") {
        Err(e) => println!("{}", e),
        Ok(pb) => println!("{}", pb.display()),
    }

    match fs::metadata("xxx/abc/klmn") {
        Err(e) => println!("{}", e),
        Ok(m) => print_metadata(&m),
    }

    // rmdir
    let e = fs::remove_dir("xxx/abc/klmn");
    check(&e);

    // rm -rf
    let e = fs::remove_dir_all("xxx");
    check(&e);

    // Result<File>
    fs::File::create("xxxyz").unwrap();

    // Result<u64>
    fs::copy("xxxyz", "abcklm").unwrap();

    let e = fs::rename("abcklm", "abcklmn");
    check(&e);

    let e = fs::remove_file("xxxyz");
    check(&e);

    let e = fs::remove_file("abcklmn");
    check(&e);
    fs::File::create("abc").unwrap();
    // link, ln
    let e = fs::hard_link("abc", "xyz");
    check(&e);

    let e = fs::remove_file("abc");
    check(&e);

    let e = fs::remove_file("xyz");
    check(&e);

    /*
    symlink_metadata(path) -> Result<Metadata>
    read_dir(path) -> Result<ReadDir>
    read_link(path) -> Result<PathBuf>
    set_permissions(path,perm)
         */
}
