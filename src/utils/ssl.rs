use crate::config::DOMAIN_NAME;
use acme2::{
    gen_rsa_private_key, AccountBuilder, AuthorizationStatus, ChallengeStatus, Csr,
    DirectoryBuilder, Error, OrderBuilder, OrderStatus,
};
use std::time::Duration;
use tokio::fs;

const LETS_ENCRYPT_URL: &str = "https://acme-v02.api.letsencrypt.org/directory";

pub async fn request() -> Result<(), Error> {
    log::info!("Requesting certificate from let's encrypt...");

    let dir = DirectoryBuilder::new(LETS_ENCRYPT_URL.to_string())
        .build()
        .await?;

    let mut account = AccountBuilder::new(dir.clone());

    account.contact(vec![]);
    account.terms_of_service_agreed(true);

    let order = OrderBuilder::new(account.build().await?)
        .add_dns_identifier((*DOMAIN_NAME).clone())
        .build()
        .await?;

    for auth in order.authorizations().await? {
        let challenge = auth.get_challenge("http-01").unwrap();
        let token = challenge.token.as_ref().unwrap();
        let contents = challenge.key_authorization()?.unwrap();

        fs::create_dir_all("/var/www/.well-known/acme-challenge")
            .await
            .unwrap();

        fs::write(
            format!("/var/www/.well-known/acme-challenge/{}", token),
            contents,
        )
        .await
        .unwrap();

        let challenge = challenge.validate().await?;
        assert_eq!(
            challenge.wait_done(Duration::from_secs(5), 3).await?.status,
            ChallengeStatus::Valid
        );
        assert_eq!(
            auth.wait_done(Duration::from_secs(5), 3).await?.status,
            AuthorizationStatus::Valid
        )
    }

    let order = order.wait_ready(Duration::from_secs(5), 3).await?;
    assert_eq!(order.status, OrderStatus::Ready);

    let private = gen_rsa_private_key(4096)?;
    let order = order.finalize(Csr::Automatic(private.clone())).await?;
    let order = order.wait_done(Duration::from_secs(5), 3).await?;
    assert_eq!(order.status, OrderStatus::Valid);

    let cert = order.certificate().await?.unwrap();
    assert!(cert.len() > 1);

    let public = &cert[0].to_pem().unwrap();
    let private = private.private_key_to_pem_pkcs8().unwrap();

    fs::write("private.pem", private)
        .await
        .expect("Couldn't private certificate key");
    fs::write("public.pem", public)
        .await
        .expect("Couldn't public certificate key");

    Ok(())
}
