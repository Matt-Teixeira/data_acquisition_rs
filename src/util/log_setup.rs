use chrono::Local;
use std::env;
use uuid::Uuid;
use std::fs::File;

pub fn log_setup() -> Result<(String, String, File), Box<dyn std::error::Error>> {
    let run_id: String = Uuid::new_v4().to_string();

    // Generate filename based on datetime_start
    let datetime_start = Local::now().format("%Y:%m:%dT%H:%M:%S.%M").to_string();
    let filename: String = format!("logs/{}_{}.json", datetime_start, env::var("APP_NAME")?);

    // Create a new log file
    let file = std::fs::File::create(filename)?;

    Ok((run_id, datetime_start, file))
}
