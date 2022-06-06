use lettre::{AsyncSmtpTransport, AsyncStd1Executor, Message};

pub fn send(email: &str) {
    let client: AsyncSmtpTransport<AsyncStd1Executor> =
        AsyncSmtpTransport::<AsyncStd1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

}
