use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::America::New_York;
use std::env;
use std::fs::File;
use uuid::Uuid;

pub fn log_setup() -> Result<(String, String, String, String, File), Box<dyn std::error::Error>> {
    let run_id: String = Uuid::new_v4().to_string();

    // Generate filename based on datetime_start
    // let datetime_start = Local::now().format("%Y:%m:%dT%H:%M:%S.%M").to_string();

    let datetime_start = get_est_timestamp();
    let log_path: String = format!("logs/{}_{}.json", datetime_start, env::var("APP_NAME")?);
    let log_name: String = format!("{}_{}.json", datetime_start, env::var("APP_NAME")?);

    println!("\ndatetime_start: {}\n", datetime_start);

    // Create a new log file
    let file = std::fs::File::create(&log_path)?;

    Ok((run_id, datetime_start, log_name, log_path, file))
}

fn get_est_timestamp() -> String {
    let utc_now: DateTime<Utc> = Utc::now();
    let eastern_now = utc_now.with_timezone(&New_York);
    eastern_now.to_rfc3339()
}
/*
LOGGING MACROS:

trace! → Very detailed logs (for debugging).

debug! → Debug information.

info! → General information (default level).

warn! → Warnings (e.g., when something is suboptimal).

error! → Errors (something went wrong).

fatal! (for panics, if needed).
*/
