use std::process::ExitCode;
use axum::http::StatusCode;

pub fn template_load_error(error: tokio::io::Error) -> ExitCode {
    eprintln!("[ERR] failed to load template!");
    eprintln!("\t{}", error.to_string());
    ExitCode::FAILURE
}

pub fn template_parse_error(error: minijinja::Error) -> ExitCode {
    eprintln!("[ERR] failed to parse template!");
    eprintln!("\t{}", error.to_string());
    ExitCode::FAILURE
}

pub fn template_render_error(error: minijinja::Error) -> (StatusCode, String) {
    let message = format!("failed to render template!\n\t{}", error.to_string());
    eprintln!("[ERR] {}", message);
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn html_tidier_error(error: tidier::Error) -> (StatusCode, String) {
    let message = format!("failed to tidy html!\n\t{}", error.to_string());
    eprintln!("[ERR] {}", message);
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn create_mailer_error(error: lettre::transport::smtp::Error) -> ExitCode {
    eprintln!("[ERR] failed create mailer!");
    eprintln!("\t{}", error.to_string());
    ExitCode::FAILURE
}

pub fn bind_listener_error(error: tokio::io::Error) -> ExitCode {
    eprintln!("[ERR] failed to tcp listener!");
    eprintln!("\t{}", error.to_string());
    ExitCode::FAILURE
}

pub fn serve_web_error(error: tokio::io::Error) -> ExitCode {
    eprintln!("[ERR] failed to serve webserver!");
    eprintln!("\t{}", error.to_string());
    ExitCode::FAILURE
}
pub fn create_message_error(error: lettre::error::Error) -> (StatusCode, String) {
    let message = format!("failed to create e-mail message!\n\t{}", error.to_string());
    eprintln!("[ERR] {}", message);
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}
pub fn send_message_error(error: lettre::transport::smtp::Error) -> (StatusCode, String) {
    let message = format!("failed to send e-mail message!\n\t{}", error.to_string());
    eprintln!("[ERR] {}", message);
    (StatusCode::INTERNAL_SERVER_ERROR, message)
}