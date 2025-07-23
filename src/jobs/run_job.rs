use crate::{boot::on_boot::AppRunState, jobs::GE};
use serde_json::json;
use tracing::{error, info};

pub async fn determine_manufacturer(
    run_id: &str,
    job_configs: AppRunState,
) -> Result<(), Box<dyn std::error::Error>> {
    let note = json!({
        "manufacturer": job_configs.config.system_manufacturer,
        "modality": job_configs.config.system_modality
    });
    info!(
        run_id,
        func = "determine_manufacturer",
        tag = "CALL",
        %note
    );

    if job_configs.config.system_manufacturer == "GE" {
        let res = GE::ge_modalities::determine_ge_modality(run_id, job_configs).await;

        match res {
            Ok(_) => {
                println!("DONE!");
            }
            Err(e) => {
                println!("Error: \n{:?}", e);
                error!(run_id, func = "determine_manufacturer", tag = "ERROR", error = ?e);
            }
        }
    }

    Ok(())
}
