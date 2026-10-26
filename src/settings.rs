use std::net::SocketAddr;
use clap::Parser;
use lettre::message::Mailbox;

#[derive(Parser, Debug)]
pub struct Settings {
    #[clap(long, env="LISTEN_ADDRESS", default_value="0.0.0.0:3000")]
    pub listen_address: SocketAddr,

    #[clap(long, env="SMTP_HOST")]
    pub smtp_host: String,

    #[clap(long, env="SMTP_PORT", default_value_t=578)]
    pub smtp_port: u16,

    #[clap(long, env="SMTP_USER")]
    pub smtp_user: String,

    #[clap(long, env="SMTP_PASS")]
    pub smtp_pass: String,

    #[clap(long, env="SMTP_SENDER")]
    pub smtp_sender: Mailbox,

    #[clap(long, env="SMTP_REPLY_TO")]
    pub smtp_reply_to: Option<Mailbox>,

    #[clap(long, env="RECEIVER")]
    pub receiver: Mailbox,

    #[clap(long, env="TEMPLATE_DIR", default_value="templates")]
    pub template_dir: String,

    #[clap(long, env="BODY_TEMPLATE_NAME", default_value="body.html")]
    pub body_template_name: String,

    #[clap(long, env="SUBJECT_TEMPLATE_NAME", default_value="subject.txt")]
    pub subject_template_name: String,

    #[clap(long, env="DANGEROUS_ACCEPT_INVALID_CERTS", default_value_t=false)]
    pub dangerous_accept_invalid_certs: bool,

    #[clap(long, env="DANGEROUS_ACCEPT_INVALID_HOSTNAMES", default_value_t=false)]
    pub dangerous_accept_invalid_hostnames: bool,
}