use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::json;
use std::net::IpAddr;
use tokio_postgres::Row;
use tracing::info;
use crate::database::models::systems_model::Systems;

#[derive(Debug, Serialize)]
pub struct GeSystems {
    pub id: Option<String>,
    pub manufacturer: Option<String>,
    pub modality: Option<String>,
    pub host_ip: Option<IpAddr>,
    pub vpn: Option<bool>,
    pub acqu_point: Option<String>,
    pub debian_server_path: Option<String>,
    pub credentials_group: Option<String>,
    pub acquisition_script: Option<String>,
    pub host_path: Option<String>,
    pub cerb_file: Option<String>,
}

impl GeSystems {
    pub fn from_row(row: Row) -> Self {
        Self {
            id: row.get("id"),
            manufacturer: row.get("manufacturer"),
            modality: row.get("modality"),
            host_ip: row.get("host_ip"),
            vpn: row.get("vpn"),
            acqu_point: row.get("acqu_point"),
            debian_server_path: row.get("debian_server_path"),
            credentials_group: row.get("credentials_group"),
            acquisition_script: row.get("acquisition_script"),
            host_path: row.get("host_path"),
            cerb_file: row.get("cerb_file"),
        }
    }

    pub async fn get_data(
        pool: &Pool,
        run_id: &str,
        boot_args: Vec<String>,
    ) -> Result<Vec<Systems>, Box<dyn std::error::Error>> {
        static GET_SYSTEMS_DATA: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/database/sql/queries/get_hhm_configs.sql"
        ));

        let client = pool.get().await?;

        let stmt = client.prepare(GET_SYSTEMS_DATA).await?;

        let manu = &boot_args[1];
        let modality = &boot_args[2];

        let rows = client.query(&stmt, &[&manu, &modality]).await?;

        let systems: Vec<Systems> = rows
            .into_iter()
            .map(|row| {
                let ge = GeSystems::from_row(row);
                Systems::Ge(ge)
            })
            .collect();

        Ok(systems)
    }
}
