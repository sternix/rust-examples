use libc::c_int;
use libc::c_void;

fn main() {
    let mut val: c_int = 0;

    let p_val = &mut val as *mut c_int as *mut c_void;

    println!("ptr: {:p}", p_val);
    println!("value: {:}", unsafe { *(p_val as *const c_int) });
}
