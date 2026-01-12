fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // eğer Some olduğunda eminsek
    // let fifth = v.pop().unwrap();

    // eğer emin değilsek
    if let Some(fifth) = v.pop() {
        assert_eq!(fifth, "105");
    }

    // 2. indekstekini verip yerine son elemanı koyuyor
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "subst".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "subst"]);
}
