use std::{fs::File, io::Read};

use crate::datatypes::stonk::Stonk;

#[allow(dead_code)]
pub fn get_stonk_from_file() -> Vec<Stonk> {
    let mut file = File::open("stonks.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let stonks: Vec<Stonk> = serde_json::from_str(&contents).unwrap();
    stonks
}
