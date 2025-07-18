use crate::util::system_structs::TunnelData;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct BounceBody {
    bounce: bool,
}

pub async fn reset_tun(
    run_log: &str,
    tunnels: Vec<TunnelData>,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let vns3_ip = env::var("VNS3_IP").expect("Missing VNS3_IP");
    let vns3_pw = env::var("VNS3_PW").expect("Missing VNS3_PW");

    println!("\nRESETTING TUNNELS");
    for tunnel in tunnels {
        if let (Some(endpoint_id), Some(tunnel_id)) = (tunnel.endpoint_id, tunnel.tunnel_id) {
            let url = format!(
                "https://{}:8000/api/ipsec/endpoints/{}/tunnels/{}",
                vns3_ip, endpoint_id, tunnel_id
            );

            let auth_header = format!(
                "Basic {}",
                general_purpose::STANDARD.encode(format!("api:{}", vns3_pw))
            );

            let res = client
                .put(&url)
                .json(&BounceBody { bounce: true })
                .header("Accept", "application/json")
                .header("Authorization", auth_header)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status() == StatusCode::OK {
                        println!(
                            "[SUCCESS] Tunnel {}-{} reset successful.",
                            endpoint_id, tunnel_id
                        );
                    } else {
                        eprintln!(
                            "[FAIL] Tunnel {}-{} failed with status: {}",
                            endpoint_id,
                            tunnel_id,
                            resp.status()
                        );
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[ERROR] Failed to reset tunnel {}-{}: {}",
                        endpoint_id, tunnel_id, e
                    );
                    return Err(Box::new(e));
                }
            }
        }
    }
    Ok(())
}

//  let auth_header = format!("Basic {}", base64::encode(format!("api:{}", vns3_pw)));
