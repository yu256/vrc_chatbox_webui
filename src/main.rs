mod endpoints;
mod responder;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use endpoints::{send_text, stop_send};
use local_ip_address::local_ip;
use responder::AppError;
use std::{env, net::UdpSocket, sync::OnceLock};
use tokio::{sync::Mutex, task::JoinHandle};
use tower_http::services::ServeFile;

static HANDLER: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();
static SOCK: OnceLock<UdpSocket> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route_service("/front", ServeFile::new("assets/index.html"))
        .route(
            "/",
            post(|req| async { send_text(req).await.map_err(AppError::from) }),
        )
        .route("/", get(stop_send));

    let port = env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(80);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    let local_ip = local_ip()?;

    if port == 80 {
        println!("http://{local_ip}/front");
    } else {
        println!("http://{local_ip}:{port}/front")
    }

    axum::serve(listener, app).await?;

    Ok(())
}
