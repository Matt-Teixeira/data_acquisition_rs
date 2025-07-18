use crate::database::models::systems_model::Systems;
use crate::jobs::GE::{CT, CV};
use crate::util::{
    get_tunnels_by_id::get_tunnels_by_id, redis::get_ip_queue, system_structs::TunnelData,
    tunnel_api::reset_tun,
};
use std::collections::HashSet;
use std::net::IpAddr;
use tokio::time::{sleep, Duration};

pub async fn reset_tunnels(
    run_id: &str,
    start_datetime: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let systems: Vec<Systems> = get_ip_queue().await?;

    println!("\n** NOt UNIQUE SYSTEMS\n{:?}", systems);

    let unique_systems = dedupe_sme(systems);

    println!("\n** UNIQUE SYSTEMS\n{:?}", unique_systems);

    // REMOVE DUPLICATE SME

    // PLACE IP ADDRESSES INTO A VEC
    let mut ip_list: Vec<&IpAddr> = Vec::new();
    for sys in unique_systems.iter() {
        if let Some(ip) = sys.host_ip() {
            ip_list.push(ip);
        }
    }

    let unique_tunnels: Vec<TunnelData> = get_tunnels_by_id(run_id, ip_list).await?;

    println!("\nunique_tunnels: \n{:?}", unique_tunnels);

    reset_tun(run_id, unique_tunnels).await?;

    println!("STARTING 5 SECOND SLEEP");
    sleep(Duration::from_secs(8)).await;
    println!("ENDING 5 SECOND SLEEP");

    let mut ge_ct_systems: Vec<Systems> = Vec::new();
    let mut ge_cv_systems: Vec<Systems> = Vec::new();
    let mut philips_ct_systems: Vec<Systems> = Vec::new();
    let mut philips_cv_systems: Vec<Systems> = Vec::new();

    for system in unique_systems {
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
        println!("\nGE CT RUN JOB:\n{:?}\n", ge_ct_systems);
        CT::get_ge_ct_files::get_ge_ct(run_id, ge_ct_systems, &start_datetime).await?;
    }
    if ge_cv_systems.len() > 0 {
        println!("\nGE CV/IR RUN JOB:\n{:?}\n", ge_cv_systems);
        CV::get_ge_cv_files::get_ge_cv(run_id, ge_cv_systems, &start_datetime).await?;
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
