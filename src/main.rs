use crate::errors::{
    bind_listener_error, create_mailer_error, create_message_error, html_tidier_error,
    send_message_error, serve_web_error, template_load_error, template_parse_error,
    template_render_error,
};
use crate::mailer::create_mailer;
use crate::settings::Settings;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use clap::Parser;
use komodo_client::entities::alert::Alert;
use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use minijinja::Environment;
use regex::Regex;
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;
use minijinja::value::Serde;
use tidier::FormatOptions;
use tokio::io::AsyncReadExt;

mod errors;
mod mailer;
mod settings;

struct AppState {
    settings: Settings,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    template_env: Environment<'static>,
    clean_regex: Regex,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    run().await.unwrap_or_else(|e| e)
}

async fn run() -> Result<ExitCode, ExitCode> {
    let settings = Settings::parse();
    let mut template_env = Environment::new();
    let clean_regex = Regex::new("\\s+").unwrap();

    let body_template = load_template(&settings, &settings.body_template_name)
        .await
        .map_err(template_load_error)?;

    let subject_template = load_template(&settings, &settings.subject_template_name)
        .await
        .map_err(template_load_error)?;

    template_env
        .add_template_owned("body".to_owned(), body_template)
        .map_err(template_parse_error)?;
    template_env
        .add_template_owned("subject".to_owned(), subject_template)
        .map_err(template_parse_error)?;

    let mailer = create_mailer(&settings).map_err(create_mailer_error)?;
    let listen_address = settings.listen_address.clone();

    // build our application with a single route
    let app = Router::new()
        .route("/", post(handle_alert))
        .with_state(Arc::new(AppState {
            settings,
            mailer,
            template_env,
            clean_regex,
        }));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(listen_address)
        .await
        .map_err(bind_listener_error)?;
    axum::serve(listener, app).await.map_err(serve_web_error)?;
    Ok(ExitCode::SUCCESS)
}

async fn handle_alert(
    State(state): State<Arc<AppState>>,
    Json(alert): Json<Alert>,
) -> Result<(), (StatusCode, String)> {
    println!("[INFO] handling {:?}", alert);
    let mut m = Message::builder().from(state.settings.smtp_sender.clone());

    if let Some(reply_to) = state.settings.smtp_reply_to.clone() {
        m = m.reply_to(reply_to);
    }

    let body_template = state.template_env.get_template("body").unwrap();
    let subject_template = state.template_env.get_template("subject").unwrap();

    let subject = subject_template
        .render(Serde(&alert))
        .map_err(template_render_error)?;
    let body = body_template
        .render(Serde(&alert))
        .map_err(template_render_error)?;

    let body = tidier::format(body, false, &FormatOptions::default()).map_err(html_tidier_error)?;

    let subject = state
        .clean_regex
        .replace_all(&subject, " ")
        .trim()
        .to_string();

    m = m.to(state.settings.receiver.clone());
    m = m.subject(subject);
    m = m.header(ContentType::TEXT_HTML);

    let m = m.body(body).map_err(create_message_error)?;

    state
        .mailer
        .send(m.into())
        .await
        .map_err(send_message_error)?;
    Ok(())
}

async fn load_template(
    settings: &Settings,
    template_name: &str,
) -> Result<String, tokio::io::Error> {
    let file = tokio::fs::File::options()
        .read(true)
        .open(Path::new(&settings.template_dir).join(template_name))
        .await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut template = String::new();
    reader.read_to_string(&mut template).await?;
    Ok(template)
}
