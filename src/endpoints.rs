use crate::{HANDLER, SOCK};
use anyhow::Result;
use axum::Json;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use serde::{Deserialize, Serialize};
use std::{
    net::{SocketAddrV4, UdpSocket},
    str::FromStr as _,
    time::Duration,
};
use tokio::{sync::Mutex, task::JoinHandle};

#[derive(Serialize, Deserialize)]
pub(super) struct Query {
    text: String,
    once: bool,
}

pub(super) async fn send_text(Json(req): Json<Query>) -> Result<&'static str> {
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

pub(super) async fn stop_send() -> &'static str {
    if let Some(handler) = HANDLER.get() {
        handler.lock().await.as_ref().map(abort);
        return "停止しました";
    }
    "現在実行されていません"
}

fn abort(handler: &JoinHandle<()>) {
    handler.abort();
}
