use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIStonk {
    pub stonk_name: String,
    pub start: u64,
    pub end: u64,
}
