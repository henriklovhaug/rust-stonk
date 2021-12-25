use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Rejection};

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type Result<T> = std::result::Result<T, Rejection>;
