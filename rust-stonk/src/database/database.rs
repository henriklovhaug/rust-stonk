use crate::datatypes::stonk::Stonk;
use rusqlite::{params, Connection, Result};

pub fn save_to_database(stonks: &Vec<Stonk>) -> Result<()> {
    let conn = Connection::open("stonks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stonks (
            id INTEGER PRIMARY KEY,
            timestamp INTEGER NOT NULL,
            open REAL NOT NULL,
            high REAL NOT NULL,
            low REAL NOT NULL,
            volume INTEGER NOT NULL,
            close REAL NOT NULL,
            adjclose REAL NOT NULL
        )",
        params![],
    )?;
    for stonk in stonks {
        conn.execute(
            "INSERT INTO stonks (timestamp, open, high, low, volume, close, adjclose)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                stonk.timestamp,
                stonk.open,
                stonk.high,
                stonk.low,
                stonk.volume,
                stonk.close,
                stonk.adjclose
            ],
        )?;
    }
    Ok(())
}
#[allow(unused_variables)]
pub fn get_stonk_from_database(stonk_name: &str) -> Result<Vec<Stonk>> {
    let conn = Connection::open("stonks.db").unwrap();
    let mut stonks: Vec<Stonk> = Vec::new();
    let mut stmp = conn.prepare("SELECT * FROM stonks")?;
    let stonk_iter = stmp.query_map([], |row| {
        Ok(Stonk {
            timestamp: row.get(1)?,
            open: row.get(2)?,
            high: row.get(3)?,
            low: row.get(4)?,
            volume: row.get(5)?,
            close: row.get(6)?,
            adjclose: row.get(7)?,
        })
    })?;
    for stonk in stonk_iter {
        stonks.push(stonk.unwrap());
    }
    Ok(stonks)
}
