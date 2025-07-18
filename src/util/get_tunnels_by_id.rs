use std::collections::HashSet;
use crate::database;
use crate::util::system_structs::TunnelData;
use std::net::IpAddr;

pub async fn get_tunnels_by_id(
    run_id: &str,
    ip_list: Vec<&IpAddr>,
) -> Result<Vec<TunnelData>, Box<dyn std::error::Error>> {
    static GET_TUNNEL_DATA: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/database/sql/queries/get_tunnels_by_ip.sql"
    ));

    let pool = database::db::create_pool(&run_id).await?;
    let client = pool.get().await?;

    let stmt = client.prepare(GET_TUNNEL_DATA).await?;

    let rows = client.query(&stmt, &[&ip_list]).await?;

    let mut tunnels: Vec<TunnelData> = rows
        .into_iter()
        .map(|row| {
            let g = TunnelData::from_row(row);
            g
        })
        .collect();

    // START: TEST SETUP
    let ip_str = "10.146.16.47";
    let ip: IpAddr = ip_str.parse()?;
    let dup_ip: TunnelData = TunnelData::new(Some(ip), Some(32), Some(68), Some(924));

    tunnels.push(dup_ip);

    // END: TEST SETUP

    let unique_tunnels: Vec<TunnelData> = dedupe_tunnels(tunnels);

    Ok(unique_tunnels)
}

fn dedupe_tunnels(tunnels: Vec<TunnelData>) -> Vec<TunnelData> {
    let mut unique = HashSet::new();
    let mut unique_tunnels: Vec<TunnelData> = Vec::new();

    for tunnel in tunnels {
        if let (Some(endpoint_id), Some(tunnel_id)) = (tunnel.endpoint_id, tunnel.tunnel_id) {
            let composite_key = format!("{}-{}", endpoint_id, tunnel_id);

            // RETURNS FALSE WHEN THE KEY IS ALREADY IN THE HashMap
            if unique.insert(composite_key) {
                // KEY NOT SEEN BEFORE, SO THIS IS A UNIQUE TUNNEL
                unique_tunnels.push(tunnel)
            }
        }
    }
    unique_tunnels
}
