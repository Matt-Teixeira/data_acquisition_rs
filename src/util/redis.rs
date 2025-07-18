use crate::database::models::{ge_model::GeSystems, systems_model::Systems};
use crate::util::system_structs::{System, SYSTEM_ONLINE};
use redis::AsyncCommands;
use serde_json::json;
use std::env;

pub async fn get_redis_kvp(key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let ip_adder = env::var("DEV_REDIS")?;

    let redis_url = format!("redis://{}/", ip_adder);

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    let val: Option<String> = con.get(key).await?;

    Ok(val)
}

pub async fn rpush_redis_queue(set_value: System) -> Result<(), Box<dyn std::error::Error>> {
    // SET UP REDIS CONNECTION
    let ip_adder = env::var("DEV_REDIS")?;
    let redis_url = format!("redis://{}/", ip_adder);
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    match &set_value {
        System::Reset(system) => {
            println!("\nTHIS IS AN IP RESET SYSTEM");
            let json = serde_json::to_string(&system)?;

            let _: () = con.rpush("rust-ip:queue", json).await?;

            Ok(())
        }
        System::Online(system) => {
            println!("\nTHIS IS AN ONLINE SYSTEM");
            let json = serde_json::to_string(&system)?;

            let _: () = con.rpush("rust-online:queue", json).await?;

            Ok(())
        }
    }
}

pub async fn get_ip_queue() -> Result<Vec<Systems>, Box<dyn std::error::Error>> {
    // SET UP REDIS CONNECTION
    let ip_adder = env::var("DEV_REDIS")?;
    let redis_url = format!("redis://{}/", ip_adder);
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    let items: Vec<String> = con.lrange("rust-ip:queue", 0, -1).await?;

    // DESERIALIZE EACH ITEM BACK INTO THE System ENUM
    let mut systems: Vec<Systems> = Vec::new();
    for item in items {
        println!("\nSYSTEM IN IP:QUEUE: {:?}", item);
        let system: Systems = serde_json::from_str(&item)?;
        systems.push(system);
    }

    Ok(systems)
}
