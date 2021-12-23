use rusqlite::{params, Connection, Result};
use crate::datatypes::stonk::Stonk;

pub fn save_to_database(stonks: &Vec<Stonk>) -> Result<()> {
    let conn = Connection::open("stonks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stonks (
            id INTEGER PRIMARY KEY,
            timestamp INTEGER NOT NULL,
            open REAL NOT NULL,
            high REAL NOT NULL,
            low REAL NOT NULL,
            volume REAL NOT NULL,
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
#[allow(dead_code)]
pub fn get_stonk_from_database(stonk_name: &str) -> Result<Vec<Stonk>> {
    let conn = Connection::open("stonks.db")?;
    let mut stonks = Vec::new();
    let mut stonk_iter = conn.prepare("SELECT * FROM stonks WHERE id = (SELECT MAX(id) FROM stonks)")?;
    let stonk_row = stonk_iter.query_row(params![], |row| {
        Ok(Stonk {
            timestamp: row.get(0)?,
            open: row.get(1)?,
            high: row.get(2)?,
            low: row.get(3)?,
            volume: row.get(4)?,
            close: row.get(5)?,
            adjclose: row.get(6)?,
        })
    })?;
    stonks.push(stonk_row);
    Ok(stonks)
}