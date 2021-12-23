use std::fs::File;
use std::io::Write;

use crate::Stonk;

pub fn save_stonk(stonk: Vec<Stonk>) {
    let mut json_string = serde_json::to_string(&stonk).unwrap();
    json_string.push_str("\n");
    let mut file = match File::create("stonks.json") {
        Ok(file) => file,
        Err(_err) => return,
    };
    file.write_all(json_string.as_bytes()).unwrap();
}
