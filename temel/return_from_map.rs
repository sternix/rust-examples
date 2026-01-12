fn main() {
    let res_ok: Result<u32, ()> = Ok(12);
    let res_err: Result<u32, ()> = Err(());

    // 12 yazıyor
    let a = res_ok.ok().map(|i| {
        println!("{i}");
        18
    });
    assert_eq!(a, Some(18));

    // ok()'den None döndüğünden birşey yazmıyor
    let b = res_err.ok().map(|i| println!("{i}"));
    assert_eq!(b, None);
}
