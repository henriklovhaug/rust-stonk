use chrono::{DateTime, TimeZone, Utc};
use files::stonksaver;
use std::time::{Duration, UNIX_EPOCH};
use tokio;
use warp::Filter;

mod database;
#[allow(unused_imports)]
use database::database::{get_stonk_from_database, save_to_database};

mod files;

mod datatypes;
use datatypes::api_stonk::APIStonk;
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
    stonk_printer(&print_stonks, "TSLA");
    stonksaver::save_stonk(stonks);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello").map(|| format!("Hello, Bing Bong STONK!"));

    // GET /STONK => 200 OK with body Very Stonk
    let stonk = warp::path!("STONK").map(|| format!("Very Stonk"));

    let stonk_name = warp::path!("STONK" / String).map(|name: String| {
        let stonk = get_stonk_from_database(&name).unwrap();
        warp::reply::json(&stonk)
    });

    let routes = hello.or(stonk).or(stonk_name);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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
