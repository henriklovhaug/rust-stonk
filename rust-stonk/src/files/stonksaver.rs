use std::fs::File;
use std::io::Write;

use yahoo_finance_api::Quote;


pub fn save_stonk(stonks: Vec<Quote>, name_of_stonk: &str) -> bool {
    let mut json_string: String = String::new();
    for stonk in stonks {
        json_string.push_str(name_of_stonk);
        json_string.push_str(&quote_to_string(stonk));

    }
    let mut file = File::create("stonks.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
    true
}

pub fn create_file() -> bool {
    let _file = match File::create("stonks.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Couldn't create file: {}", e);
            return false;
        }
    };
    true
}

fn quote_to_string(quote: Quote) -> String {
    let mut small_string = String::new();
    small_string.push_str(quote.timestamp.to_string().as_str());
    small_string.push_str(stringify!(quote.open));
    small_string.push_str(stringify!(quote.high));
    small_string.push_str(stringify!(quote.low));
    small_string.push_str(stringify!(quote.close));
    small_string.push_str(stringify!(quote.volume));
    small_string.push_str(stringify!(quote.adj_close));
    small_string
}