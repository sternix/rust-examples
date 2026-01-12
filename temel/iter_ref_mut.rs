fn messages() -> Vec<String> {
    vec![
        "msg1".to_string(),
        "msg2".to_string(),
        "msg3".to_string(),
        "msg4".to_string(),
        "msg5".to_string(),
    ]
}

fn main() {
    let msgs = messages();
    for msg in msgs {
        // msgs'un tüm elemenlarının sahipliği sırasıyla msg'e taşınıyor
        println!("{msg}");
        // ve msg burada drop ediliyor
    }

    // HATA value borrowed here after move
    //println!("{}", msgs.len());

    let msgs = messages();
    for msg in &msgs {
        // şimdi sadece referansları alınıyor
        println!("String {:?} is at address {:p}", *msg, msg);
    }

    // Bu çalışıyor
    println!("Len: {}", msgs.len());

    let mut msgs = messages();
    for msg in &mut msgs {
        // şimdi sadece referansları alınıyor
        msg.push('!'); // mul olduğundan değişiklik yapabiliyoruz
        println!("String {:?} is at address {:p}", *msg, msg);
    }

    // msgs hala yaşıyor
    println!("Len: {}", msgs.len());
}
