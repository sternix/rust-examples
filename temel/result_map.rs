fn main() {
    let res_ok: Result<u32, ()> = Ok(12);
    let res_err: Result<u32, ()> = Err(());

    // 12 yazıyor
    res_ok.ok().map(|i| println!("{i}"));

    // ok()'den None döndüğünden birşey yazmıyor
    res_err.ok().map(|i| println!("{i}"));
}
