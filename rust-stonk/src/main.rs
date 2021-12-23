use chrono::{DateTime, TimeZone, Utc};
use files::stonksaver;
use std::time::{Duration, UNIX_EPOCH};
use tokio;
use yahoo_finance_api as yahoo;
mod database;
use database::database::{get_stonk_from_database, save_to_database};

mod files;

mod datatypes;
use datatypes::stonk::Stonk;

mod stonk_finder;
use stonk_finder::stonk_finder::get_stonk_history;

#[tokio::main]
async fn main() {
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    let stonks: Vec<Stonk> = get_stonk_history("TSLA", start, end).await;
    //stonk_printer(&stonk, "TSLA");
    //save_to_database(&stonk).ok();
    let print_stonks = get_stonk_from_database("TSLA").unwrap();
    stonk_printer(&stonks, "TSLA");
    stonksaver::save_stonk(stonks);
}

fn stonk_printer(stonks: &Vec<Stonk>, stonk_name: &str) {
    for stonk in stonks {
        let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(stonk.timestamp));
        println!("{} at {}:", stonk_name, time.format("%Y-%m-%d %H:%M:%S"));
        println!(
            "\t Opening price: {} \n\t Daily high: {} \n\t Daily low: {} \n\t Volume: {} \n\t Closing price: {} \n\t Adjusted closing price: {}
                  ",
            stonk.open, stonk.high, stonk.low, stonk.volume, stonk.close, stonk.adjclose
        );
    }
}
