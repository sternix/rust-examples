// array'ların boyutları aynı olmadığından panikliyor,

pub fn vulnerable_memcopy(dest: &mut [u8], src: &[u8], n: usize) {
    let mut i = 0;

    while i < n {
        dest[i] = src[i];
        i += 1;
    }
}

fn main() {
    let a: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3];
    let mut b: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    vulnerable_memcopy(&mut b, &a, 12);
}
