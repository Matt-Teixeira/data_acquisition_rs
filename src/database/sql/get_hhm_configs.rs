use crate::database::models::{systems_model::Systems, ge_model::GeSystems, philips_model::PhilipsSystems};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::json;
use tracing::info;
//use tracing::instrument;

pub async fn get_system_configs(
    pool: &Pool,
    run_id: &str,
    boot_args: Vec<String>,
) -> Result<Vec<Systems>, Box<dyn std::error::Error>> {
    // LOG: START
    let func = "get_all_systems";
    let tag = "CALL";
    info!(run_id, func, tag);
    // LOG: END

    match boot_args[1].as_str() {
        "GE" => GeSystems::get_data(pool, run_id, boot_args).await,
        "Philips" => PhilipsSystems::get_data(pool, run_id, boot_args).await,
        _ => Ok(vec![]),
    }

    /*
    // LOG: START
    let note = json!({"systems": systems});
    let tag = "DETAILS";
    info!(run_id, func, tag, note = %note);
    // LOG: END
    */

    // Ok(systems)
}
