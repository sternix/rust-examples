struct Mail {
    from: String,
    to: String,
    subject: String,
    body: String,
    user: String,
    password: String,
    server: String,
}

fn main() {
    let mail = Mail {
        from: "from@example.com".into(),
        to: "to@example.com".into(),
        subject: "subject".into(),
        body: "body".into(),
        user: "user".into(),
        password: "password".into(),
        server: "server".into(),
    };

    destructure(&mail);

    destructure2(&mail);
}

// partial destructure
fn destructure(mail: &Mail) {
    let Mail { from, user, .. } = mail;
    // sadece from ve user alanlarını ayırdık
    // sıra önemli değil
    println!("From: {from}, user: {user}");
}

// partial destructure alanları farklı bir değişkene atadık
fn destructure2(mail: &Mail) {
    // from -> kimden, subject -> konu olarak alındı
    let Mail {
        from: kimden,
        subject: konu,
        ..
    } = mail;
    println!("Kimden: {kimden}, Konu: {konu}");
}

