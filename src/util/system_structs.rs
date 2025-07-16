use crate::database::models::systems_model::Systems;
use serde::{Deserialize, Serialize};

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