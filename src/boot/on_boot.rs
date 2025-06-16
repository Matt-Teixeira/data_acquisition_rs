use crate::database;
use crate::database::models::systems_model::Systems;
use tracing::{error, info};

#[derive(Debug)]
struct AppRunConfig {
    system_manufacturer: String,
    system_modality: String,
}

#[derive(Debug)]
struct AppRunState {
    config: AppRunConfig,
    systems: Vec<Systems>,
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
    let app = AppRunState::new(run_id, boot_args).await;

    match app {
        Ok(data) => {
            for system in data.systems {
                match system {
                    Systems::Ge(ge) => {
                        println!("\n{:?}", ge);
                    }
                    Systems::Philips(philips) => {
                        println!("\n{:?}", philips);
                    }
                }
            }
            return Ok(true);
        }
        Err(e) => {
            error!(run_id, error = ?e);
            return Err(e);
        }
    }
}
