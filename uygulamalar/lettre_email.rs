/*
[dependencies]
lettre = "0.10.0-rc.4"
lettre_email = "0.9"
*/

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("Ben<test@example.com.tr>".parse().unwrap())
        .reply_to("Test <test@example.com.tr>".parse().unwrap())
        .to("to@example.com.tr".parse().unwrap())
        .subject("Subject")
        .body(String::from("Body!"))
        .unwrap();

    let creds = Credentials::new("test".to_string(), "password".to_string());

    // Open a remote connection to server
    let mailer = SmtpTransport::relay("mail.example.com.tr")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
