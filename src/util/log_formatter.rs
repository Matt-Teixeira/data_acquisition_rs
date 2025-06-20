use chrono::Local;
use std::process::Stdio;
use tokio::process::Command;

pub async fn format_log(log_name: String, log_path: String) -> Result<(), Box<dyn std::error::Error>>  {
    let script_path = "/home/matt-teixeira/education/rust/rust-azure/src/read/tools/format_log.sh";
    let dir_name = Local::now().format("%Y-%m-%d").to_string();
    let args: Vec<&String> = vec![&log_path, &log_name, &dir_name];

    println!("jq -s . {} > ./logs/{}/{}", &log_path, &log_name, &dir_name);

    let output = Command::new(script_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT: {}", stdout);
    println!("STDERR: {}", stderr);

    Ok(())
}
