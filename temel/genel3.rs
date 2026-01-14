fn main() {
    assert_eq!(
        "root:0:Charlie Root:".split(':').collect::<Vec<_>>(),
        vec!["root", "0", "Charlie Root", ""]
    );
    // dikkat: Charlie Root'dan sonra bir boşluk daha var

    assert_eq!(
        "127.0.0.1 localhost\n\
         127.0.0.1 www.google.com\n"
            .split_terminator('\n')
            .collect::<Vec<_>>(),
        vec!["127.0.0.1 localhost", "127.0.0.1 www.google.com"]
    );
    // dikkat burada ektra bir boşluk yok

    let poem = "This is just to say\n\
                I have eaten\n\
                the plums\n\
                again\n";

    assert_eq!(
        poem.split_whitespace().collect::<Vec<_>>(),
        vec![
            "This", "is", "just", "to", "say", "I", "have", "eaten", "the", "plums", "again"
        ]
    );

    assert_eq!("\t*.rs    ".trim(), "*.rs");
    assert_eq!("\t*.rs    ".trim_start(), "*.rs    ");
    assert_eq!("\t*.rs    ".trim_end(), "\t*.rs");

    assert_eq!("001990".trim_start_matches('0'), "1990");

    // ı ve i 'de sorun var locale gerekiyor herhalde
    assert_eq!("üğşçö".to_uppercase(), "ÜĞŞÇÖ");
    assert_eq!("ÜĞŞÇÖ".to_lowercase(), "üğşçö");
}
