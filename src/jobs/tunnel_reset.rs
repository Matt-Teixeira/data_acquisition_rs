use crate::database::models::systems_model::Systems;
use crate::jobs::GE::{CT, CV, MRI};
use crate::util::{
    get_tunnels_by_id::get_tunnels_by_id,
    redis::{delete_queue, get_ip_queue},
    system_structs::TunnelData,
    tunnel_api::reset_tun,
};
use serde_json::json;
use std::collections::HashSet;
use std::net::IpAddr;
use tokio::time::{sleep, Duration};
use tracing::info;

pub async fn reset_tunnels(
    run_id: &str,
    start_datetime: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let func: &str = "reset_tunnels";
    info!(run_id, func, tag = "CALL");

    let systems: Vec<Systems> = get_ip_queue().await?;

    // DELETE QUEUE
    delete_queue("rust-ip:queue").await?;

    // REMOVE DUPLICATE SME/SYSTEM
    let unique_systems = dedupe_sme(systems);

    let note = json!({
        "systems": &unique_systems
    });
    info!(
        run_id,
        func,
        tag = "DETAILS",
        %note
    );

    // PLACE IP ADDRESSES INTO A VEC
    let mut ip_list: Vec<&IpAddr> = Vec::new();
    for sys in unique_systems.iter() {
        if let Some(ip) = sys.host_ip() {
            ip_list.push(ip);
        }
    }

    // REMOVE DUPLICATE TUNNELS
    let unique_tunnels: Vec<TunnelData> = get_tunnels_by_id(run_id, ip_list).await?;

    let note = json!({
        "tunnels": &unique_tunnels
    });
    info!(
        run_id,
        func,
        tag = "DETAILS",
        %note
    );

    reset_tun(run_id, unique_tunnels).await?;

    println!("STARTING 8 SECOND SLEEP");
    sleep(Duration::from_secs(8)).await;
    println!("ENDING 8 SECOND SLEEP");

    let mut ge_ct_systems: Vec<Systems> = Vec::new();
    let mut ge_cv_systems: Vec<Systems> = Vec::new();
    let mut ge_mri_systems: Vec<Systems> = Vec::new();
    let mut philips_ct_systems: Vec<Systems> = Vec::new();
    let mut philips_cv_systems: Vec<Systems> = Vec::new();

    for system in unique_systems {
        match system {
            Systems::Ge(ref sys) => {
                if sys.modality.as_deref() == Some("CT") {
                    ge_ct_systems.push(system);
                } else if sys.modality.as_deref() == Some("CV/IR") {
                    ge_cv_systems.push(system);
                } else {
                    ge_mri_systems.push(system);
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
        CT::get_ge_ct_files::get_ge_ct(run_id, ge_ct_systems, &start_datetime).await?;
    }
    if ge_cv_systems.len() > 0 {
        CV::get_ge_cv_files::get_ge_cv(run_id, ge_cv_systems, &start_datetime).await?;
    }
    if ge_mri_systems.len() > 0 {
        MRI::get_ge_mri_files::get_ge_mri(run_id, ge_mri_systems, &start_datetime).await?;
    }

    return Ok(());
}

fn dedupe_sme(systems: Vec<Systems>) -> Vec<Systems> {
    let mut unique = HashSet::new();
    let mut unique_sme: Vec<Systems> = Vec::new();

    for system in systems {
        if let Some(sme) = system.get_sme() {
            if unique.insert(sme.clone()) {
                unique_sme.push(system)
            }
        }
    }
    unique_sme
}
