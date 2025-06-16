use deadpool_postgres::{Config, Pool, Runtime};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::env;
use std::fs;
use tracing::info;

pub async fn create_pool(run_id: &str) -> Result<Pool, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let func = "create_pool";
    let tag = "CALL";
    info!(run_id, func, tag);

    let mut cfg = Config::new();
    cfg.host = Some(env::var("PG_HOST")?);
    cfg.user = Some(env::var("PG_USER")?);
    cfg.password = Some(env::var("PG_PW")?);
    cfg.dbname = Some(env::var("PG_DB")?);
    cfg.port = Some(env::var("PG_PORT")?.parse()?);
    cfg.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    let cert_data = fs::read(env::var("PGSSLROOTCERT")?)?;
    let cert = Certificate::from_pem(&cert_data)?;
    let tls_connector = TlsConnector::builder().add_root_certificate(cert).build()?;
    let tls = MakeTlsConnector::new(tls_connector);

    let pool = cfg.create_pool(Some(Runtime::Tokio1), tls)?;

    Ok(pool)
}
