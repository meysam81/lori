use samotop::mail::{Builder, DebugService, MailDir, Name};
use samotop::server::TcpServer;
use samotop::smtp::{Esmtp, Prudence, SmtpParser};
use std::io::Read;

use std::path::{Path, PathBuf};
use std::time::Duration;

fn prudence() -> Prudence {
    Prudence::default()
        .with_read_timeout(Duration::from_millis(3000))
        .with_banner_delay(Duration::from_millis(1000))
}

async fn send_to_sendgrid(mail: mail_parser::Message<'_>, api_key: &String) {
    use sendgrid::v3::*;

    let _to_name = match mail.to() {
        mail_parser::HeaderValue::Address(addr) => addr.name.clone().unwrap_or_default(),
        _ => panic!("to is not an address"),
    };
    let to_email = match mail.to() {
        mail_parser::HeaderValue::Address(addr) => addr.address.clone().unwrap(),
        _ => panic!("to is not an address"),
    };
    let _from_name = match mail.from() {
        mail_parser::HeaderValue::Address(addr) => addr.name.clone().unwrap_or_default(),
        _ => panic!("from is not an address"),
    };
    let from_email = match mail.from() {
        mail_parser::HeaderValue::Address(addr) => addr.address.clone().unwrap(),
        _ => panic!("from is not an address"),
    };
    let subject = mail.subject().unwrap();
    let body = mail.body_text(0).unwrap().to_string();

    let p = Personalization::new(Email::new(to_email));

    let m = Message::new(Email::new(from_email))
        .set_subject(subject)
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(&body),
        )
        .add_personalization(p);

    let sender = Sender::new(api_key.into());
    let resp = sender.send(&m).await.unwrap();

    dbg!(resp);
}

async fn parse_and_log_mails(mail_dir: &Path, sendgrid_api_key: &String) {
    use mail_parser::*;

    let new_dir = mail_dir.join("new");

    std::fs::create_dir_all(&new_dir).unwrap();

    let files = std::fs::read_dir(new_dir).unwrap();
    for file in files {
        dbg!(&file);
        let file = file.unwrap();
        let mut f = std::fs::File::open(file.path()).unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();

        let mail = Message::parse(&buf[..]).unwrap();
        send_to_sendgrid(mail, sendgrid_api_key).await;

        std::fs::remove_file(file.path()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mail_dir: PathBuf = "/tmp/maildir".into();

    let service = Builder
        + Name::new("localhost")
        + DebugService
        + Esmtp.with(SmtpParser)
        + prudence()
        // + Spf
        + MailDir::new(mail_dir.clone())
        .unwrap();

    // if let Some(cfg) = setup.tls_config().await? {
    //     service += EsmtpStartTls.with(SmtpParser, RustlsProvider::from(TlsAcceptor::from(cfg)));
    // }

    let sendgrid_api_key: String =
        std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY env var cannot be empty");
    let addr = std::env::var("BIND_ADDR").unwrap_or("0.0.0.0:2525".into());

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            parse_and_log_mails(&mail_dir, &sendgrid_api_key).await;
        }
    });

    TcpServer::on_all(vec![addr])
        .serve(service.build())
        .await
        .unwrap();
}
