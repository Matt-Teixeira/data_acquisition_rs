use crate::database::models::{ge_model::GeSystems, systems_model::Systems};
use crate::util::{
    redis, stderr_regex,
    system_structs::{System, SYSTEM_ONLINE},
};
use futures::future::join_all;
use serde_json::json;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_ge_cv(
    run_id: &str,
    sys_configs: Vec<Systems>,
    start_datetime: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let func = "get_ge_cv";
    info!(run_id = run_id, func, tag = "CALL");

    let jobs: Vec<_> = sys_configs
        .into_iter()
        .filter_map(|system| {
            if let Systems::Ge(ge_system) = system {
                Some(job(run_id, ge_system, &start_datetime))
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

async fn job(
    run_id: &str,
    mut ge_system: GeSystems,
    start_datetime: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let job_id: String = Uuid::new_v4().to_string();
    let func = "job";

    // CREATE PATH TO POINT TO ACQUISITION SCRIPT .sh
    let bash_script = ge_system
        .acquisition_script
        .as_ref()
        .ok_or("no bash script")?;
    let script_path = format!(
        "/home/matt-teixeira/education/rust/rust-azure/src/read/GE/CV/{}",
        bash_script
    );

    let system_id = ge_system.id.clone().ok_or("no system id")?;

    // SET BASH ARGS
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

    // CREATE ARGS LIST
    let args: Vec<&String> = vec![&host_ip, &user_name, &pass, &debian_path];

    // PASS ARGS AND EXECUTE ACQUISITION SCRIPT
    let output = Command::new(script_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // START: CHECK STDOUT/STDERR AND DO WORK IF CONNECTION ERRORS
    // UNSUCCESSFULL CONNECTION ATTEMPT
    if !output.status.success() {
        // RESET THIS SYSTEM'S TUNNEL - PUSH TO ip:queue
        if !ge_system.tunnel_reset {
            ge_system.tunnel_reset = true;
            redis::rpush_redis_queue(System::Reset(Systems::Ge(ge_system))).await?;
        } else if ge_system.tunnel_reset {
            let connection_error = stderr_regex::ge_ct_stderr_check(&stderr)?;

            let system_to_queue = SYSTEM_ONLINE::new(
                system_id.to_string(),
                String::from(start_datetime),
                false,
                String::from("hhm"),
                false,
                connection_error,
            );

            redis::rpush_redis_queue(System::Online(system_to_queue)).await?;
        }
        let note = json!({
            "system_id": &system_id,
            "host_ip": &host_ip,
            "debian_path": &debian_path,
            "stdout": stdout,
            "stderr": stderr
        });
        error!(run_id = run_id, job_id = &job_id, func, tag = "ERROR", note = %note);
    }
    // SUCCESSFULL CONNECTION ATTEMPT
    else {
        // SET ERROR TO null via None ENUM
        let connection_error = None;

        // USE STRUCT TO CREATE VALUE TO BE INSERTED INTIO REDIS rust-online:queue
        let system_to_queue = SYSTEM_ONLINE::new(
            system_id.to_string(),
            String::from(start_datetime),
            false,
            String::from("hhm"),
            true,
            connection_error,
        );

        // SEND TO rust-online:queue TO THEN BE INSERTED INTO alert.offline_hhm_conn TABLE
        redis::rpush_redis_queue(System::Online(system_to_queue)).await?;

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
