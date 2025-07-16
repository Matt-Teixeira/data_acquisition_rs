use crate::{boot::on_boot::AppRunState, jobs::GE};
use serde_json::json;
use tracing::info;

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
        GE::ge_modalities::determine_ge_modality(run_id, job_configs).await?;
    }

    Ok(())
}
