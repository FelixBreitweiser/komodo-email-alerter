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
use std::sync::Arc;
use tidier::FormatOptions;
use tokio::io::AsyncReadExt;

mod mailer;
mod settings;

struct AppState {
    settings: Settings,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    template_env: Environment<'static>,
    clean_regex: Regex,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let settings = Settings::parse();
    let mut template_env = Environment::new();
    let clean_regex = Regex::new("\\s+").unwrap();

    let body_template = load_template(&settings, &settings.body_template_name)
        .await
        .expect("failed to load body template");
    let subject_template = load_template(&settings, &settings.subject_template_name)
        .await
        .expect("failed to load subject template");

    template_env
        .add_template_owned("body".to_owned(), body_template)
        .expect("syntax error in body template");
    template_env
        .add_template_owned("subject".to_owned(), subject_template)
        .expect("syntax error in subject template");

    let mailer = create_mailer(&settings).expect("failed to create mailer");

    // build our application with a single route
    let app = Router::new()
        .route("/", post(handle_alert))
        .with_state(Arc::new(AppState {
            settings,
            mailer,
            template_env,
            clean_regex,
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_alert(
    State(state): State<Arc<AppState>>,
    Json(alert): Json<Alert>,
) -> Result<(), (StatusCode, String)> {
    println!("handling {:?}", alert);
    let mut m = Message::builder().from(state.settings.smtp_sender.clone());

    if let Some(reply_to) = state.settings.smtp_reply_to.clone() {
        m = m.reply_to(reply_to);
    }

    let body_template = state.template_env.get_template("body").unwrap();
    let subject_template = state.template_env.get_template("subject").unwrap();

    let subject = subject_template
        .render(&alert)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let body = body_template
        .render(&alert)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let body = tidier::format(body, false, &FormatOptions::default())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let subject = state
        .clean_regex
        .replace_all(&subject, " ")
        .trim()
        .to_string();

    m = m.to(state.settings.receiver.clone());
    m = m.subject(subject);
    m = m.header(ContentType::TEXT_HTML);

    let m = m
        .body(body)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    state
        .mailer
        .send(m.into())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
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
