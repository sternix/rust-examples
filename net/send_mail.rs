// https://medium.com/turkiye-rust-community/rust-ile-mail-g%C3%B6nderimi-1f07202fbbe7

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    // SMTP sunucusu bilgileri
    let smtp_server = "smtp.example.com";
    let smtp_port = 587;
    let smtp_username = "your_username";
    let smtp_password = "your_password";

    // E-posta detayları
    let from_email = "sender@example.com";
    let to_email = "recipient@example.com";
    let subject = "Hello from Rust!";
    let body = "This is a test email sent from Rust using direct SMTP.";

    // SMTP sunucusuna bağlanma
    let mut stream = TcpStream::connect(format!("{}:{}", smtp_server, smtp_port))
        .expect("Failed to connect to server");
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));

    // SMTP sunucusu ile iletişim
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // Giriş komutları
    stream
        .write_all(b"EHLO example.com\r\n")
        .expect("Failed to send EHLO command");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // Kimlik doğrulama komutları
    stream
        .write_all(b"AUTH LOGIN\r\n")
        .expect("Failed to send AUTH LOGIN command");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // Kullanıcı adı ve şifre kodlanmış şekilde gönderilir
    stream
        .write_all(format!("{}\r\n", base64::encode(smtp_username)).as_bytes())
        .expect("Failed to send username");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    stream
        .write_all(format!("{}\r\n", base64::encode(smtp_password)).as_bytes())
        .expect("Failed to send password");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // Gönderme komutları
    stream
        .write_all(format!("MAIL FROM:<{}>\r\n", from_email).as_bytes())
        .expect("Failed to send MAIL FROM command");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    stream
        .write_all(format!("RCPT TO:<{}>\r\n", to_email).as_bytes())
        .expect("Failed to send RCPT TO command");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    stream
        .write_all(b"DATA\r\n")
        .expect("Failed to send DATA command");
    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // E-posta içeriği gönderilir
    stream
        .write_all(format!("From: {}\r\n", from_email).as_bytes())
        .expect("Failed to send From header");
    stream
        .write_all(format!("To: {}\r\n", to_email).as_bytes())
        .expect("Failed to send To header");
    stream
        .write_all(format!("Subject: {}\r\n", subject).as_bytes())
        .expect("Failed to send Subject header");
    stream
        .write_all(b"\r\n")
        .expect("Failed to send empty line");
    stream
        .write_all(body.as_bytes())
        .expect("Failed to send email body");
    stream
        .write_all(b"\r\n.\r\n")
        .expect("Failed to send email end marker");

    let response = reader.fill_buf().expect("Failed to read response");
    println!("Server response: {:?}", String::from_utf8_lossy(response));

    // Bağlantıyı kapat
    stream
        .write_all(b"QUIT\r\n")
        .expect("Failed to send QUIT command");
}
