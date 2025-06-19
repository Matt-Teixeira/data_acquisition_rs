use crate::{boot::on_boot::AppRunState, jobs::GE::CT};

pub async fn determine_ge_modality(run_id: &str, job_configs: AppRunState) -> Result<(), Box<dyn std::error::Error>> {
    println!("JOB CONFIG: {:?}", job_configs);
    CT::get_ge_ct_files::get_ge_ct(run_id, job_configs.systems).await?;

    Ok(())
}
