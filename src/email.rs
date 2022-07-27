use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_email(log: &String) {
    dotenv().ok();

    let email = Message::builder()
        .from(env::var("EMAIL").unwrap().parse().unwrap())
        .to(env::var("EMAIL").unwrap().parse().unwrap())
        .subject("Backup")
        .body(log.to_string())
        .unwrap();

    let creds = Credentials::new(env::var("EMAIL").unwrap(), env::var("PASSWORD").unwrap());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
