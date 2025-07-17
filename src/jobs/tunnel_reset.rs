use crate::boot::on_boot::AppRunState;
use crate::database::models::{
    ge_model::GeSystems, philips_model::PhilipsSystems, systems_model::Systems,
};
use crate::jobs::GE::{CT, CV};
use crate::util::redis::get_ip_queue;

pub async fn reset_tunnels(
    run_id: &str,
    start_datetime: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nHELLO TUNNEL RESETS!!!\n");
    let systems: Vec<Systems> = get_ip_queue().await?;

    /*
       DO WORK TO RESET TUNNELS
    */

    let mut ge_ct_systems: Vec<Systems> = Vec::new();
    let mut ge_cv_systems: Vec<Systems> = Vec::new();
    let mut philips_ct_systems: Vec<Systems> = Vec::new();
    let mut philips_cv_systems: Vec<Systems> = Vec::new();

    for system in systems {
        match system {
            Systems::Ge(ref sys) => {
                if sys.modality.as_deref() == Some("CT") {
                    ge_ct_systems.push(system);
                } else if sys.modality.as_deref() == Some("CV/IR") {
                    ge_cv_systems.push(system);
                }
            }
            Systems::Philips(ref sys) => {
                if sys.modality.as_deref() == Some("CT") {
                    philips_ct_systems.push(system);
                } else if sys.modality.as_deref() == Some("CV/IR") {
                    philips_cv_systems.push(system);
                }
            }
        }
    }

    if ge_ct_systems.len() > 0 {
        println!("\nGE CT:\n{:?}\n", ge_ct_systems);
        CT::get_ge_ct_files::get_ge_ct(run_id, ge_ct_systems, &start_datetime).await?;
    }
    if ge_cv_systems.len() > 0 {
        println!("\nGE CV/IR:\n{:?}\n", ge_cv_systems);
        CV::get_ge_cv_files::get_ge_cv(run_id, ge_cv_systems, &start_datetime).await?;
    }

    return Ok(());
}
