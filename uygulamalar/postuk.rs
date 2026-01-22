/*

[dependencies]
anyhow = "1.0"
clap = "4.5.54"
lettre = "0.11.19"
*/

use anyhow::Error;
use clap::{Arg, ArgAction, Command};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

struct Mail {
    from: String,
    to: String,
    subject: String,
    body: String,
    user: String,
    password: String,
    server: String,
}

fn main() -> Result<(), Error> {
    let matches = Command::new("postuk")
        .version("0.1.0")
        .author("Yavuz Tanrıverdi")
        .about("Command line mail sender")
        .arg(
            Arg::new("server")
                .long("server")
                .required(true)
                .action(ArgAction::Set)
                .help("Mail sunucu ip yada domaini"),
        )
        .arg(
            Arg::new("user")
                .long("user")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("from")
                .long("from")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("to")
                .long("to")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("subject")
                .long("subject")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("body")
                .long("body")
                .required(true)
                .action(ArgAction::Set),
        )
        .get_matches();

    let mail = Mail {
        from: matches.get_one::<String>("from").unwrap().clone(),
        to: matches.get_one::<String>("to").unwrap().clone(),
        subject: matches.get_one::<String>("subject").unwrap().clone(),
        body: matches.get_one::<String>("body").unwrap().clone(),
        user: matches.get_one::<String>("user").unwrap().clone(),
        password: matches.get_one::<String>("password").unwrap().clone(),
        server: matches.get_one::<String>("server").unwrap().clone(),
    };

    match send_mail(mail) {
        Ok(_) => println!("Başarı ile gönderildi"),
        Err(e) => println!("Hata: {e}"),
    }

    Ok(())
}

fn send_mail(mail: Mail) -> Result<(), Error> {
    let email = Message::builder()
        .from(mail.from.parse()?)
        .to(mail.to.parse()?)
        .subject(mail.subject)
        .header(ContentType::TEXT_HTML)
        .body(String::from(mail.body))?;

    let creds = Credentials::new(mail.user, mail.password);

    let mailer = SmtpTransport::relay(&mail.server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(From::from(e)),
    }
}

/*

postuk \
    --server=mail.example.com
    --user=mail.user
    --password='secret'
    --from=mail.user@example.com
    --to=people@example.com
    --subject='This is mail subject'
    --body='This is email body with html <b>BOLD</b>'

*/
