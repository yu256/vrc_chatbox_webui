mod responder;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Json, Router,
};
use local_ip_address::local_ip;
use responder::AppError;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use serde::{Deserialize, Serialize};
use std::{
    env,
    net::{SocketAddrV4, UdpSocket},
    str::FromStr as _,
    sync::OnceLock,
    time::Duration,
};
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

#[derive(Serialize, Deserialize)]
pub(crate) struct Query {
    text: String,
    once: bool,
}

async fn send_text(Json(req): Json<Query>) -> Result<&'static str> {
    if let Some(handler) = HANDLER.get() {
        handler.lock().await.as_ref().map(abort);
    }

    let sock = SOCK.get_or_init(|| UdpSocket::bind("127.0.0.1:9002").unwrap());
    let to_addr = SocketAddrV4::from_str("127.0.0.1:9000")?;

    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/chatbox/input".to_string(),
        args: vec![OscType::String(req.text), OscType::Bool(true)],
    }))?;

    sock.send_to(&msg_buf, to_addr)?;

    if !req.once {
        *HANDLER.get_or_init(|| Mutex::new(None)).lock().await = Some(tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                sock.send_to(&msg_buf, to_addr).unwrap();
            }
        }));
    }

    Ok("ok")
}

async fn stop_send() -> &'static str {
    if let Some(handler) = HANDLER.get() {
        handler.lock().await.as_ref().map(abort);
        return "停止しました";
    }
    "現在実行されていません"
}

fn abort(handler: &JoinHandle<()>) {
    handler.abort();
}
