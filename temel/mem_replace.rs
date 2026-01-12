struct Person {
    name: Option<String>,
}

fn main() {
    let mut persons = vec![
        Person {
            name: Some("xyz".to_string()),
        },
        Person {
            name: Some("klm".to_string()),
        },
        Person {
            name: Some("prs".to_string()),
        },
    ];

    /* HATA
    if let Some(name) = persons[0].name {
        println!("name: {}", name);
    }
    */

    // 0. elemanın name alanını None yapıp eski değeri döndürüyor,
    let first_name = std::mem::replace(&mut persons[0].name, None);
    println!("{}", first_name.unwrap());

    // yukarıdaki işlemi yapıyor
    // sonuçta 1. eleman'ın name alanı None oluyor
    let first_name = persons[1].name.take();
    println!("{}", first_name.unwrap());

    for p in &persons {
        match p.name {
            Some(ref name) => println!("{}", name),
            None => println!("NONE"),
        }
    }
}
