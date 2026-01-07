/*
tokio = { version = "1", features = ["full"] }

lettre = { version = "0.10.0-rc.4", default-features = false, features = ["smtp-transport", "tokio1-rustls-tls", "hostname", "builder"] }
*/

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};

#[tokio::main]
async fn main() {
    let email = Message::builder()
        .from("Ben<test@example.com.tr>".parse().unwrap())
        .reply_to("Test <reply@domain.com.tr>".parse().unwrap())
        .to("to@example.com.tr".parse().unwrap())
        .subject("Subject")
        .body(String::from("Body!"))
        .unwrap();

    let creds = Credentials::new("test".to_string(), "secret".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("mail.example.com.tr")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
