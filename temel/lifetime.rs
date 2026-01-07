// lifetime'lar illa 'a 'b olmasına gerek yok
// fakat aski bir kelime olmalı
// küçük harflerden oluşması önerilir - uyarı veriyor

fn fn_a<'tr>(i: &'tr i32) -> &'tr i32 {
    i
}

trait T<'ankara> {
    fn x(&self, &'ankara i32) -> &'ankara i32;
}

struct S;

impl<'giresun> T<'giresun> for S {
    fn x(&self, i: &'giresun i32) -> &'giresun i32 {
        i
    }
}

fn main() {
    let x = 10;

    assert_eq!(*fn_a(&x), 10);

    let s = S;
    assert_eq!(*s.x(&x), 10);
}
