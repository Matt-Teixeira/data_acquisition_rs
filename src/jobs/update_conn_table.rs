use crate::database::db::create_pool;
use crate::util::{redis::get_online_queue, system_structs::System};
use chrono::{DateTime, FixedOffset};

pub async fn update_conn_hhm(run_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let systems: Vec<System> = get_online_queue().await?;

    let pool = create_pool(run_id).await?;
    let client = pool.get().await?;

    for system in systems {
        if let System::Online(sys) = system {
            let parsed_dt: DateTime<FixedOffset> = sys.capture_datetime.parse()?;
            // let utc_dt = parsed_dt.with_timezone(&Utc);

            client
                .execute(
                    "
            INSERT INTO alert.offline_hhm_conn (
                system_id,
                capture_datetime,
                host_intervention,
                connetion_error,
                successful_acquisition
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (system_id) DO UPDATE SET
                capture_datetime = EXCLUDED.capture_datetime,
                host_intervention = EXCLUDED.host_intervention,
                connetion_error = EXCLUDED.connetion_error,
                successful_acquisition = EXCLUDED.successful_acquisition,
                inserted_at = EXCLUDED.inserted_at
        ",
                    &[
                        &sys.id,
                        &parsed_dt,
                        &sys.host_intervention,
                        &sys.connetion_error,
                        &sys.successful_acquisition,
                    ],
                )
                .await?;
        }
    }
    Ok(true)
}
