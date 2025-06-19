use crate::{
    database::models::systems_model::Systems,
    util::decrypt::{self, decrypt},
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::json;
use std::net::IpAddr;
use tokio_postgres::Row;
use tracing::{error, info};

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
    pub user_enc: Option<String>,
    pub password_enc: Option<String>,
    pub user: String,
    pub password: String,
}

// GET DB CONFIG DATA
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
            user_enc: row.get("user_enc"),
            password_enc: row.get("password_enc"),
            user: String::new(),
            password: String::new(),
        }
    }

    pub async fn get_db_config(
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
                let mut ge = GeSystems::from_row(row);
                match ge.decrypt_creds() {
                    Ok((user, pass)) => {
                        ge.user = user;
                        ge.password = pass;

                        let note = json!({
                            "system_id": &ge.id,
                            "message": "Credentials Decrypted"
                        });
                        info!(run_id = run_id, note = %note);
                    }
                    Err(e) => {
                        error!(run_id = run_id, error = ?e);
                    }
                }
                Systems::Ge(ge)
            })
            .collect();

        Ok(systems)
    }
}

// TODO: BRING THIS LOGIC INTO WHERE THE DB COME IN
impl GeSystems {
    pub fn decrypt_creds(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        let sme = &self.id;
        match sme {
            Some(data) => println!("SME: {}", data),
            None => println!("No SME Data"),
        }

        // .as_ref() Converts from Option<String> to Option<&String> — avoids moving the String
        // .ok_or("Error Message") Converts the Option into a Result where None becomes an Err()
        let user_enc = self.user_enc.as_ref().ok_or("Missind user cred")?;
        let user = decrypt(user_enc)?;

        let pass_enc = self.password_enc.as_ref().ok_or("Missind password cred")?;
        let password = decrypt(pass_enc)?;

        Ok((user, password))
    }
}
