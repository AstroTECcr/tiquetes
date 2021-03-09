use std::path;

use lettre::{Transport, SmtpClient};
use lettre::smtp::authentication::IntoCredentials;
use lettre_email::{EmailBuilder, mime};

pub struct MailSender<'a> {
    smtp_address: &'a str,
    username: &'a str,
    password: &'a str,
}

impl<'a> MailSender<'a> {
    pub fn new(smtp_address: &'a str, username: &'a str, password: &'a str,) -> Self {
        MailSender {
            smtp_address,
            username,
            password,
        }
    }

    // esto apesta pero bueno, no hay tiempo de hacerlo bien lol
    pub fn send_astrotec_ticket(self, to: &str, subject: &str, text: &str, attachment: path::PathBuf, ) {
        let email = EmailBuilder::new()
            .to(to)
            .from(self.username)
            .subject(subject)
            .text(text)
            .attachment_from_file(&attachment, attachment.to_str(), &mime::IMAGE_PNG).unwrap()
            .build()
            .unwrap()
            .into();
        let credentials = (self.username, self.password).into_credentials();
        let mut client = SmtpClient::new_simple(self.smtp_address)
            .unwrap()
            .credentials(credentials)
            .transport();
        let _result = client.send(email);
    }

}
