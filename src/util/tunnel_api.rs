use crate::util::system_structs::TunnelData;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::json;
use std::env;
use tracing::{error, info, warn};

#[derive(Serialize)]
struct BounceBody {
    bounce: bool,
}

pub async fn reset_tun(
    run_id: &str,
    tunnels: Vec<TunnelData>,
) -> Result<(), Box<dyn std::error::Error>> {
    let func: &str = "reset_tun";
    info!(run_id, func, tag = "CALL");

    dotenv().ok();

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let vns3_ip = env::var("VNS3_IP").expect("Missing VNS3_IP");
    let vns3_pw = env::var("VNS3_PW").expect("Missing VNS3_PW");

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
                        let note = json!({
                            "tunnel": &tunnel,
                            "res_status": "[SUCCESS] Tunnel reset successful"
                        });
                        info!(
                            run_id,
                            func,
                            tag = "DETAILS",
                            %note
                        );
                    } else {
                        let note = json!({
                            "tunnel": &tunnel,
                            "res_status": "[UNSUCCESSFUL] Tunnel reset unsuccessful"
                        });
                        warn!(
                            run_id,
                            func,
                            tag = "DETAILS",
                            %note
                        );
                    }
                }
                Err(e) => {
                    let note = json!({
                        "tunnel": &tunnel,
                        "res_status": "[UNSUCCESSFUL] Tunnel reset unsuccessful"
                    });
                    error!(
                        run_id,
                        func,
                        tag = "DETAILS",
                        error = ?e,
                        %note
                    );
                    return Err(Box::new(e));
                }
            }
        }
    }
    Ok(())
}

//  let auth_header = format!("Basic {}", base64::encode(format!("api:{}", vns3_pw)));
