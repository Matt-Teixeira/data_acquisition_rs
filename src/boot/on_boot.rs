use crate::jobs;
use crate::{database, database::models::systems_model::Systems};
use serde::Serialize;
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct AppRunConfig {
    pub system_manufacturer: String,
    pub system_modality: String,
}

#[derive(Debug, Serialize)]
pub struct AppRunState {
    pub config: AppRunConfig,
    pub systems: Vec<Systems>,
}

impl AppRunState {
    async fn new(run_id: &str, boot_args: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppRunConfig {
            system_manufacturer: boot_args[1].clone(),
            system_modality: boot_args[2].clone(),
        };

        let pool = database::db::create_pool(&run_id).await?;
        let systems =
            database::sql::get_hhm_configs::get_system_configs(&pool, &run_id, boot_args).await?;

        Ok(AppRunState { config, systems })
    }
}

pub async fn on_boot(
    run_id: &str,
    boot_args: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let func: &str = "on_boot";
    let app = AppRunState::new(run_id, boot_args).await;

    match app {
        Ok(job_configs) => {
            let note = json!({
                "job_configs": job_configs
            });
            info!(
                run_id,
                func,
                tag = "DETAILS",
                %note
            );
            jobs::run_job::determine_manufacturer(run_id, job_configs).await?;
            return Ok(true);
        }
        Err(e) => {
            error!(run_id, func, error = ?e);
            return Err(e);
        }
    }
}
