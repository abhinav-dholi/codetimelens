use chrono::{DateTime, Utc, NaiveDate};
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub start: String,
    pub end: String,
    pub project_path: String,
}

pub fn init_db() -> Result<()> {
    let conn = Connection::open("codetimelens.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            project_path TEXT NOT NULL,
            files TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn save_session(start: DateTime<Utc>, end: DateTime<Utc>, project_path: &str, files: &str) -> Result<()> {
    let conn = Connection::open("codetimelens.db")?;
    conn.execute(
        "INSERT INTO sessions (start_time, end_time, project_path, files) VALUES (?1, ?2, ?3, ?4)",
        params![start.to_rfc3339(), end.to_rfc3339(), project_path, files],
    )?;
    Ok(())
}

pub fn get_history() -> Result<Vec<SessionEntry>> {
    let conn = Connection::open("codetimelens.db")?;
    let mut stmt = conn.prepare("SELECT start_time, end_time, project_path FROM sessions")?;

    let sessions = stmt
        .query_map([], |row| {
            Ok(SessionEntry {
                start: row.get(0)?,
                end: row.get(1)?,
                project_path: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(sessions)
}

pub fn group_sessions_by_day() -> HashMap<NaiveDate, Vec<SessionEntry>> {
    let mut map = HashMap::new();
    if let Ok(history) = get_history() {
        for entry in history {
            if let Ok(start) = DateTime::parse_from_rfc3339(&entry.start) {
                let date = start.date_naive();
                map.entry(date).or_insert(vec![]).push(entry);
            }
        }
    }
    map
}

pub fn export_sessions(format: &str) -> Result<(), Box<dyn Error>> {
    let history = get_history()?;

    match format {
        "json" => {
            let file = File::create("export.json")?;
            serde_json::to_writer_pretty(file, &history)?;
        }
        "csv" => {
            let mut wtr = csv::Writer::from_path("export.csv")?;
            for s in history {
                wtr.serialize((s.start, s.end, s.project_path.clone()))?;
            }
            wtr.flush()?;
        }
        _ => println!("Unsupported format. Use 'json' or 'csv'."),
    }

    Ok(())
}