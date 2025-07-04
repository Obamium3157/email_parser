use std::net::TcpStream;

use imap::types::Fetch;
use mailparse::{MailHeaderMap, parse_mail};
use native_tls::TlsConnector;

fn main() -> imap::error::Result<()> {
    let email = "p.m.zhdanov@yandex.ru";
    let password = "noeqaraudaqcggjo";

    let tls = TlsConnector::builder().build().unwrap();
    let client = {
        let tcp = TcpStream::connect("imap.yandex.ru:993")?;
        let tls_stream = tls.connect("imap.yandex.ru", tcp)?;
        imap::Client::new(tls_stream)
    };

    let mut session = client.login(email, password).map_err(|e| e.0)?;
    println!("Successfully logged in!");

    session.select("INBOX")?;

    let fetches = session.fetch("1:*", "BODY[]")?;
    let all_fetches: &Vec<Fetch> = fetches.as_ref();
    for fetch in all_fetches.iter().rev().take(5) {
        if let Some(body) = fetch.body() {
            let parsed = parse_mail(body).expect("Failed parsing an email");

            let subject = parsed
                .headers
                .get_first_value("Subject")
                .unwrap_or_default();
            let from = parsed.headers.get_first_value("From").unwrap_or_default();

            if from.split_whitespace().next().unwrap().contains("tilda.ws") {
                println!("From: {}", from);
                println!("Subject: {}", subject);

                println!("Subparts number: {}", parsed.subparts.len());

                if parsed.subparts.len() == 0 {
                    let body_text = parsed.get_body().unwrap_or_default();

                    println!("{}", body_text);
                } else {
                    if let Some(sub) = parsed.subparts.iter().find(|p| {
                        p.ctype.mimetype == "text/plain" || p.ctype.mimetype == "text/html"
                    }) {
                        if let Ok(text) = sub.get_body() {
                            println!("Body: ({}):\n{}", sub.ctype.mimetype, text);
                        }
                    }
                }
            }
        }
    }

    session.logout()?;
    Ok(())
}
