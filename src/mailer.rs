use crate::settings::Settings;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::{AsyncSmtpTransport, Error};
use lettre::Tokio1Executor;
use lettre::transport::smtp::client::{Tls, TlsParameters};

#[cfg(not(debug_assertions))]
pub fn create_mailer(settings: &Settings) -> Result<AsyncSmtpTransport<Tokio1Executor>, Error> {
    let creds = Credentials::new(settings.smtp_user.clone(), settings.smtp_pass.clone());

    let tls = TlsParameters::builder(settings.smtp_host.clone())
        .build()?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_host)?
        .tls(Tls::Required(tls))
        .port(settings.smtp_port)
        .credentials(creds)
        .build();

    Ok(mailer)
}

#[cfg(debug_assertions)]
pub fn create_mailer(settings: &Settings) -> Result<AsyncSmtpTransport<Tokio1Executor>, Error> {
    let creds = Credentials::new(settings.smtp_user.clone(), settings.smtp_pass.clone());

    let tls = TlsParameters::builder(settings.smtp_host.clone())
        .dangerous_accept_invalid_certs(settings.dangerous_accept_invalid_certs)
        .dangerous_accept_invalid_hostnames(settings.dangerous_accept_invalid_hostnames)
        .build()?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_host)?
        .tls(Tls::Required(tls))
        .port(settings.smtp_port)
        .credentials(creds)
        .build();

    Ok(mailer)
}
