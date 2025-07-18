use crate::database::models::systems_model::Systems;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug)]
pub enum System {
    Online(SYSTEM_ONLINE),
    Reset(Systems),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SYSTEM_ONLINE {
    pub id: String,
    pub capture_datetime: String,
    pub successful_acquisition: bool,
    pub data_source: String,
    pub host_intervention: bool,
    pub connetion_error: Option<String>,
}

impl SYSTEM_ONLINE {
    pub fn new(
        id: String,
        capture_datetime: String,
        successful_acquisition: bool,
        data_source: String,
        host_intervention: bool,
        connetion_error: Option<String>,
    ) -> Self {
        Self {
            id,
            capture_datetime,
            successful_acquisition,
            data_source,
            host_intervention,
            connetion_error,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TunnelData {
    pub remote_subnet_ip: Option<IpAddr>,
    pub remote_subnet_mask: Option<i32>,
    pub endpoint_id: Option<i32>,
    pub tunnel_id: Option<i32>,
}

impl TunnelData {
    pub fn from_row(row: Row) -> Self {
        Self {
            remote_subnet_ip: row.get("remote_subnet_ip"),
            remote_subnet_mask: row.get("remote_subnet_mask"),
            endpoint_id: row.get("endpoint_id"),
            tunnel_id: row.get("tunnel_id"),
        }
    }

    pub fn new(
        remote_subnet_ip: Option<IpAddr>,
        remote_subnet_mask: Option<i32>,
        endpoint_id: Option<i32>,
        tunnel_id: Option<i32>,
    ) -> Self {
        Self {
            remote_subnet_ip,
            remote_subnet_mask,
            endpoint_id,
            tunnel_id,
        }
    }
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct SYSTEM_RESET {
    pub id: String,
    pub manufacturer: String,
    pub modality: String,
    pub host_ip: String,
    pub debian_server_path: String,
    pub credentials_group: String,
    pub acquisition_script: String,
    pub data_source: String,
    pub tunnel_reset: bool,
}

impl SYSTEM_RESET {
    pub fn new(
        id: String,
        manufacturer: String,
        modality: String,
        host_ip: String,
        debian_server_path: String,
        credentials_group: String,
        acquisition_script: String,
        data_source: String,
        tunnel_reset: bool,
    ) -> Self {
        Self {
            id,
            manufacturer,
            modality,
            host_ip,
            debian_server_path,
            credentials_group,
            acquisition_script,
            data_source,
            tunnel_reset,
        }
    }
}
 */
