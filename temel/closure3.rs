use std::collections::HashMap;

fn main() {
    let mut dict = HashMap::new();
    dict.insert("a", "1");
    dict.insert("b", "2");
    dict.insert("c", "3");
    dict.insert("d", "4");

    let dump_dict = || {
        // burada aşağıdakinden farklı olarak dict, &dict olarak değiştirildi
        // closure FnOnce'dan Fn'e dönüştü
        // şimdi istediğimiz kadar çağırabiliriz.
        for (key, value) in &dict {
            println!("{}-{}", key, value);
        }
    };

    dump_dict();
    dump_dict();

    let dump_dict = || {
        // burada &dict değil dict olduğundan move ediliyor
        for (key, value) in dict {
            println!("{}-{}", key, value);
        }
    };

    dump_dict();
    // dump_dict();   <- bu hata verir,
}
