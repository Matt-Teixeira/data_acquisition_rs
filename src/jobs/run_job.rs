use crate::{boot::on_boot::AppRunState, jobs::GE};

pub async fn determine_manufacturer(run_id: &str, job_configs: AppRunState) -> Result<(), Box<dyn std::error::Error>> {
    if job_configs.config.system_manufacturer == "GE".to_string() {
        println!("{:?}", job_configs.config);
        GE::ge_modalities::determine_ge_modality(run_id, job_configs).await?;
    }

    Ok(())
}
