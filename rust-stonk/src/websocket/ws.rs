use crate::{Client, Clients};
use crate::stonk_finder::stonk_finder::get_stonk_history;
use chrono::{Utc, TimeZone};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };
    clients.lock().await.insert(uuid.clone(), new_client);
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients).await;
    }
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}
async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    if message == "ping" || message == "ping\n" {
        let locked = clients.lock().await;
        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
            }
            None => return,
        }
        return;
    }

    else if message.starts_with("stonk") {
        let locked = clients.lock().await;
        let stonk_string = message.split_ascii_whitespace().nth(1).unwrap();
        match locked.get(client_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("sending stonk");
                    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
                    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
                    let send_string = get_stonk_history(stonk_string, start, end).await;

                    let _ = sender.send(Ok(Message::text(serde_json::to_string(&send_string).unwrap())));
                }
            }
            None => return,
        }
        return;
    };
}
