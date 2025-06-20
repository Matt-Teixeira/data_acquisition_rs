mod boot;
mod database;
mod jobs;
mod util;
use dotenv::dotenv;
use serde_json::json;
use std::env;
use tracing::{error, info, warn};
use tracing_appender::non_blocking;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let boot_args: Vec<String> = env::args().collect();

    // LOG: START
    let (run_id, _start_datetime, log_name, log_path, file) = util::log_setup::log_setup()?;
    let app_name = env::var("APP_NAME")?;
    let func = "main";
    let tag = "CALL";

    println!("{}", log_name);
    println!("{}", log_path);

    // INIT LOG SETUP
    let (non_blocking, _guard) = non_blocking(file);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false) // turn off color codes in file
        .json()
        .init();

    if boot_args.len() < 2 {
        let note = json!({"message": "less than 2 command line args supplied"});
        let tag = "WARNING";
        warn!(
            run_id,
            app_name,
            func,
            tag,
            %note
        );
        return Ok(());
    }

    info!(
        run_id,
        arg_1 = boot_args[1],
        arg_2 = boot_args[2],
        app_name,
        func,
        tag
    );
    // LOG: END

    // RUN JOBS: START
    let successful_run = boot::on_boot::on_boot(&run_id, boot_args.clone()).await;
    // RUN JOBS: END

    match successful_run {
        Ok(v) => {
            let tag = "DETAILS";
            let note = json!({"message": "job complete"});
            info!(
                run_id,
                arg_1 = boot_args[1],
                arg_2 = boot_args[2],
                app_name,
                func,
                tag,
                %note
            );
            println!("Run Successfully Complete: {}", v)
        }
        Err(e) => {
            let tag = "ERROR";
            error!(
                run_id,
                arg_1 = boot_args[1],
                arg_2 = boot_args[2],
                app_name,
                func,
                tag,
                error = ?e
            );
            println!("{}", e)
        }
    }

    let _formated_log = util::log_formatter::format_log(log_name, log_path).await?;

    return Ok(());
}
