use async_smtp::{
    smtp::ConnectionReuseParameters, ClientSecurity, EmailAddress, Envelope, SendableEmail,
    SmtpClient, Transport,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_simple_send(c: &mut Criterion) {
    let mut sender = SmtpClient::new("127.0.0.1:2525", ClientSecurity::None)
        .unwrap()
        .transport();

    c.bench_function("send email", move |b| {
        b.iter(|| {
            let email = SendableEmail::new(
                Envelope::new(
                    Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
                    vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
                )
                .unwrap(),
                "id".to_string(),
                "Hello ß☺ example".to_string().into_bytes(),
            );
            let result = black_box(async_std::task::block_on(async {
                sender.send(email).await
            }));
            result.unwrap();
        })
    });
}

fn bench_reuse_send(c: &mut Criterion) {
    let mut sender = SmtpClient::new("127.0.0.1:2525", ClientSecurity::None)
        .unwrap()
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();
    c.bench_function("send email with connection reuse", move |b| {
        b.iter(|| {
            let email = SendableEmail::new(
                Envelope::new(
                    Some(EmailAddress::new("user@localhost".to_string()).unwrap()),
                    vec![EmailAddress::new("root@localhost".to_string()).unwrap()],
                )
                .unwrap(),
                "id".to_string(),
                "Hello ß☺ example".to_string().into_bytes(),
            );
            let result = black_box(async_std::task::block_on(async {
                sender.send(email).await
            }));
            result.unwrap();
        })
    });
}

criterion_group!(benches, bench_simple_send, bench_reuse_send);
criterion_main!(benches);
