use crate::database::models::systems_model::Systems;
use crate::util::system_structs::{System, SYSTEM_ONLINE};
use redis::AsyncCommands;
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

    // DESERIALIZE EACH ITEM BACK INTO THE Systems ENUM
    let mut systems: Vec<Systems> = Vec::new();
    for item in items {
        println!("\nSYSTEM IN IP:QUEUE: {:?}", item);
        let system: Systems = serde_json::from_str(&item)?;
        systems.push(system);
    }

    Ok(systems)
}

pub async fn get_online_queue() -> Result<Vec<System>, Box<dyn std::error::Error>> {
    // SET UP REDIS CONNECTION
    let ip_adder = env::var("DEV_REDIS")?;
    let redis_url = format!("redis://{}/", ip_adder);
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    let queue_items: Vec<String> = con.lrange("rust-online:queue", 0, -1).await?;

    // DESERIALIZE EACH ITEM BACK INTO THE System ENUM
    let mut systems: Vec<System> = Vec::new();
    for item in queue_items {
        match serde_json::from_str::<SYSTEM_ONLINE>(&item) {
            Ok(parsed) => systems.push(System::Online(parsed)),
            Err(e) => eprintln!("Deserialization error: {e}\nJSON: {item}"),
        }
    }

    println!("\nONLINE QUEUE:\n{:?}", systems);

    Ok(systems)
}

pub async fn delete_queue(queue_to_del: &str) -> Result<(), Box<dyn std::error::Error>> {
    // SET UP REDIS CONNECTION
    let ip_adder = env::var("DEV_REDIS")?;
    let redis_url = format!("redis://{}/", ip_adder);
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_async_connection().await?;

    // Explicit type annotation avoids fallback warning: returns in value as number of rows deleted.
    con.del::<_, ()>(queue_to_del).await?;
    Ok(())
}
