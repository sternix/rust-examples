use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    // normal ekleme
    m.insert("x", 1);
    m.insert("y", 2);
    m.insert("z", 3);

    // eğer key varsa, eski değeri Option ile veriyor
    assert_eq!(m.insert("z", 4).unwrap(), 3);

    // yukarıdaki insert değil key olduğundan update yaptı
    assert_eq!(m.len(), 3);

    assert_eq!(m["x"], 1);
    assert_eq!(m.get("y").unwrap(), &2);

    if let Some(e) = m.get_mut("z") {
        *e = 28;
    }

    assert_eq!(m["z"], 28);

    assert_eq!(m.contains_key("x"), true);
    assert_eq!(m.remove("z").unwrap(), 28);
    assert_eq!(m.remove("z").is_none(), true);

    // --edition=2018 ile bunsuz derleniyor
    {
        let entry = m.entry("x");
        // eğer yoksa panikliyor
        assert_eq!(entry.key(), &"x");
    }

    {
        m.entry("!").or_insert(123);
    }
    assert_eq!(m["!"], 123);

    {
        m.entry("?").or_insert_with(|| 456);
    }

    assert_eq!(m["?"], 456);

    {
        m.entry("?").and_modify(|e| *e += 1);
    }

    assert_eq!(m["?"], 457);

    {
        m.entry("#").and_modify(|e| *e += 2).or_insert(789);
    }

    assert_eq!(m["#"], 789);

    {
        m.entry("#").and_modify(|e| *e += 2).or_insert(789);
    }

    assert_eq!(m["#"], 791);

    m.clear();
    assert_eq!(m.len(), 0);

    /*
    let m: HashMap<usize,i32> = (0..10).enumerate().collect();
    for (k,v) in &m {
        println!("{} -> {}", k,v);
    }
     */
}
