use crate::datatypes::api_stonk::{APIStonk, LocalApiStonk};
use crate::datatypes::client::{Client, Clients};
use crate::datatypes::stonk::SearchStonk;
use crate::stonk_finder::stonk_finder::{
    find_stonk_by_company_name, get_last_stonk, get_latest_stonks, get_stonk_history,
};

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
    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(client) => {
            if let Some(sender) = client.sender.as_ref() {
                match message {
                    "ping" | "ping\n" => {
                        let _ = sender.send(Ok(Message::text("pong")));
                    }

                    _ if message.starts_with("stonk") => {
                        let stonk_name = message.split_ascii_whitespace().nth(1).unwrap();
                        let stonk_history = match get_latest_stonks(stonk_name).await {
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
                        let search_term = match message.split_ascii_whitespace().nth(1) {
                            Some(v) => v,
                            None => {
                                let _ = sender
                                    .send(Ok(Message::text("error: no search term provided")));
                                return;
                            }
                        };
                        let search_results = find_stonk_by_company_name(search_term).await;
                        let send_string: Vec<SearchStonk> = search_results
                            .iter()
                            .map(|x| SearchStonk::from(x))
                            .collect();
                        let _ = sender.send(Ok(Message::text(
                            serde_json::to_string(&send_string).unwrap(),
                        )));
                    }

                    _ if message.starts_with("now") => {
                        let stonk_name = match message.split_ascii_whitespace().nth(1) {
                            Some(v) => v,
                            None => {
                                let _ =
                                    sender.send(Ok(Message::text("error: no stonk name provided")));
                                return;
                            }
                        };
                        let stonk_history = match get_last_stonk(stonk_name).await {
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

                    _ if message.starts_with("{") => {
                        let value = client_msg_objects(&message).await;
                        if value.is_ok() {
                            let _ = sender.send(Ok(Message::text(value.unwrap())));
                        } else {
                            let _ = sender.send(Ok(Message::text("error parsing message object")));
                        }
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

//Handle objects sent from client
async fn client_msg_objects(message: &str) -> Result<String, String> {
    match message {
        _ if serde_json::from_str::<APIStonk>(message).is_ok() => {
            let stonk = serde_json::from_str::<APIStonk>(message).unwrap();
            let rust_stonk = LocalApiStonk::from(&stonk);
            let history =
                match get_stonk_history(&rust_stonk.stonk_name, rust_stonk.start, rust_stonk.end)
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        println!("error getting stonk history: {}", e);
                        return Err("Error getting stonk history".to_string());
                    }
                };
            let send_string = serde_json::to_string(&history).unwrap();

            return Ok(send_string);
        }

        _ => {
            println!("error parsing message: {}", message);
            return Err("Error parsing message".to_string());
        }
    };
}
