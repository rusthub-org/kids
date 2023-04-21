use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::authentication::Credentials,
};
use serde_json::{Map, json};

use crate::util::{constant::CFG, common::get_lang_msg};

pub async fn send_email(
    language: String,
    user_id: String,
    username: String,
    nickname: String,
    email_to: String,
) {
    let domain = CFG.get("DOMAIN").unwrap();
    let email_smtp = CFG.get("EMAIL_SMTP").unwrap();
    let email_from = dotenv::var("EMAIL_FROM").unwrap();
    let email_username = dotenv::var("EMAIL_USERNAME").unwrap();
    let email_password = dotenv::var("EMAIL_PASSWORD").unwrap();

    let mut email_args = Map::new();
    email_args.insert("nickname".to_string(), json!(nickname));
    email_args.insert("username".to_string(), json!(username));

    let email_subject = get_lang_msg(
        language.as_str(),
        "register",
        "email-subject",
        Some(&email_args),
    );

    email_args.insert("domain".to_string(), json!(domain));
    email_args.insert("language".to_string(), json!(language));
    email_args.insert("user_id".to_string(), json!(user_id));

    let email_body = get_lang_msg(
        language.as_str(),
        "register",
        "email-body",
        Some(&email_args),
    );
    let email_body = email_body
        .replace("\u{2068}", "")
        .replace("\u{2069}", "")
        .replace("<br><br>", "");

    let email_message = Message::builder()
        .from(email_from.parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject(email_subject)
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(email_username, email_password);
    let mailer =
        SmtpTransport::relay(email_smtp).unwrap().credentials(creds).build();

    match mailer.send(&email_message) {
        Ok(_) => println!("\n\n\nEmail sent successfully!\n\n\n"),
        Err(e) => println!("\n\n\nCould not send email: {:?}\n\n\n", e),
    }
}
