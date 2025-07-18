use crate::jobs;
use crate::{database, database::models::systems_model::Systems};
use serde::Serialize;
use serde_json::json;
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct AppRunConfig {
    pub system_manufacturer: String,
    pub system_modality: String,
    pub start_datetime: String,
}

#[derive(Debug, Serialize)]
pub struct AppRunState {
    pub config: AppRunConfig,
    pub systems: Vec<Systems>,
}

impl AppRunState {
    async fn new(
        run_id: &str,
        boot_args: Vec<String>,
        start_datetime: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppRunConfig {
            system_manufacturer: boot_args[1].clone(),
            system_modality: boot_args[2].clone(),
            start_datetime,
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
    start_datetime: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    if boot_args[1] == "ip_reset" {
        jobs::tunnel_reset::reset_tunnels(run_id, start_datetime).await?;
        return Ok(true);
    } else {
        let func: &str = "on_boot";
        let app = AppRunState::new(run_id, boot_args, start_datetime).await;

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

                let job_status = jobs::run_job::determine_manufacturer(run_id, job_configs).await;
                match job_status {
                    Ok(_) => {
                        return Ok(true);
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(e) => {
                error!(run_id, func, error = ?e);
                return Err(e);
            }
        }
    }
}
