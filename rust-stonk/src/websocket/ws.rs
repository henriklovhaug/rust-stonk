use crate::datatypes::stonk::SearchStonk;
use crate::stonk_finder::stonk_finder::{find_stonk_by_company_name, get_stonk_history};
use crate::{Client, Clients};
use chrono::{TimeZone, Utc};
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

//TODO for Stefan:
// - reduce complexity of client_msg
async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(client) => {
            if let Some(sender) = client.sender.as_ref() {
                match message {
                    "ping" | "ping\n" => {
                        let _ = sender.send(Ok(Message::text("pong")));
                    }

                    //Check if message starts with "stonk:"
                    //TODO support user specified start and end dates
                    _ if message.starts_with("stonk") => {
                        let stonk_name = message.split_ascii_whitespace().nth(1).unwrap();
                        let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
                        let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
                        let stonk_history = match get_stonk_history(stonk_name, start, end).await {
                            Ok(v) => v,
                            Err(e) => {
                                println!("error getting stonk history: {}", e);
                                let _ =
                                    sender.send(Ok(Message::text("error getting stonk history")));
                                return;
                            }
                        };
                        let _ = sender.send(Ok(Message::text(
                            serde_json::to_string(&stonk_history).unwrap(),
                        )));
                    }

                    _ if message.starts_with("search") => {
                        let search_term = &message.split_ascii_whitespace().nth(1).unwrap();
                        let search_results = find_stonk_by_company_name(search_term).await;
                        let send_string: Vec<SearchStonk> = search_results
                            .iter()
                            .map(|x| SearchStonk::from(x))
                            .collect();
                        let _ = sender.send(Ok(Message::text(
                            serde_json::to_string(&send_string).unwrap(),
                        )));
                    }
                    _ => {
                        let _ = sender.send(Ok(Message::text("Not yet supported")));
                    }
                }
            }
        }
        None => return,
    };
    return;
}
