use crate::{boot::on_boot::AppRunState, jobs::GE::CT};
use serde_json::json;
use tracing::info;

pub async fn determine_ge_modality(
    run_id: &str,
    job_configs: AppRunState,
) -> Result<(), Box<dyn std::error::Error>> {
    let note = json!({
        "manufacturer": job_configs.config.system_manufacturer,
        "modality": job_configs.config.system_modality
    });
    info!(
        run_id,
        func = "determine_ge_modality",
        tag = "CALL",
        %note
    );
    CT::get_ge_ct_files::get_ge_ct(run_id, job_configs.systems).await?;

    Ok(())
}
