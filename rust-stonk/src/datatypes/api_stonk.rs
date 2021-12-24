use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIStonk {
    pub stonk_name: String,
}
