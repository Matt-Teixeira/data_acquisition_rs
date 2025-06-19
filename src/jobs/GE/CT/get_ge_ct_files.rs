use crate::database::models::{ge_model::GeSystems, systems_model::Systems};
use futures::future::join_all;
use serde_json::json;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_ge_ct(
    run_id: &str,
    sys_configs: Vec<Systems>,
) -> Result<(), Box<dyn std::error::Error>> {
    let func = "get_ge_ct";
    info!(run_id = run_id, func, tag = "CALL");

    let jobs: Vec<_> = sys_configs
        .into_iter()
        .filter_map(|system| {
            if let Systems::Ge(ge_system) = system {
                Some(job(run_id, ge_system))
            } else {
                None
            }
        })
        .collect();

    let results = join_all(jobs).await;

    // Handle results
    for result in results {
        match result {
            Ok(_) => println!("Job completed successfully"),
            Err(e) => eprintln!("Job failed: {}", e),
        }
    }

    Ok(())
}

async fn job(run_id: &str, ge_system: GeSystems) -> Result<(), Box<dyn std::error::Error>> {
    let job_id: String = Uuid::new_v4().to_string();
    let func = "job";
    let bash_script = ge_system
        .acquisition_script
        .as_ref()
        .ok_or("no bash script")?;
    let script_path = format!(
        "/home/matt-teixeira/education/rust/rust-azure/src/read/GE/CT/{}",
        bash_script
    );
    let system_id = ge_system.id.as_ref().ok_or("no system id")?;

    let host_ip = ge_system.host_ip.as_ref().ok_or("no host ip")?.to_string(); //$1
    let user_name = &ge_system.user; // $2
    let pass = &ge_system.password; // $3
    let debian_path = format!(
        "/home/matt-teixeira/education/rust/rust-azure/files/{}",
        &system_id
    ); // $4

    let note = json!({
        "system_id": &system_id,
        "host_ip": &host_ip,
        "debian_path": &debian_path
    });

    info!(run_id = run_id, job_id = &job_id, func, tag = "DETAILS", note = %&note);

    let args: Vec<&String> = vec![&host_ip, &user_name, &pass, &debian_path];

    let output = Command::new(script_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("Script failed: {}", stderr);
        let note = json!({
            "system_id": &system_id,
            "host_ip": &host_ip,
            "debian_path": &debian_path,
            "stdout": stdout,
            "stderr": stderr
        });
        error!(run_id = run_id, job_id = &job_id, func, tag = "ERROR", note = %note);
    } else {
        let note = json!({
            "system_id": &system_id,
            "host_ip": &host_ip,
            "debian_path": &debian_path,
            "stdout": stdout,
            "stderr": stderr
        });

        info!(run_id = run_id, job_id = job_id, func, tag = "DETAILS", note = %note);
    }

    Ok(())
}

/*
currently giving job function ownership of ge_system.
Can changes this to pass in reference of ge_system by doing:

Some(async move {
    job(run_id, &ge_system).await // all inside async block
})

1) async { ... } creates a future.
2) move tells Rust: move all captured variables into the future's scope.
*/
