fn main() {
    let x = 5;

    let raw = &x as *const i32;
    println!("{}", unsafe { *raw });

    // burada adresini okuduğumuzdan unsafe'e gerek yok
    println!("{:p}", raw);

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;
    unsafe {
        *raw_mut = 28;
    }
    println!("{}", unsafe { *raw_mut });

    println!("{:p}", raw_mut);
    println!("{:p}", &y);

    // explicit cast
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // implicit coercion
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    let ref_imm: &u32;
    let ref_mut: &mut u32;

    // burada unsafe'den safe'e dönüyoruz
    unsafe {
        ref_imm = &*p_imm;
        ref_mut = &mut *p_mut;
    }

    assert_eq!(*ref_imm, 1);
    assert_eq!(*ref_mut, 2);
}
