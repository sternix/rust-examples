/*
. operator implicitly dereferences its left operand if needed.
bundan dolayı (*instance).field yerine instance.field yazabiliyoruz.
*/

struct Kisi {
    adi: &'static str,
}

fn main() {
    let k = Kisi { adi: "Test" };

    let kisi_ref = &k;
    assert_eq!(kisi_ref.adi, "Test");
    // ikiside aynı
    assert_eq!((*kisi_ref).adi, "Test");
}
