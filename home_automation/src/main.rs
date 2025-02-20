use std::env;
use std::sync::Arc;
use tapo::{ApiClient, PlugEnergyMonitoringHandler};
use tokio;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let device = Arc::new(get_device().await.expect("Failed to connect to device"));
    let on_device = Arc::clone(&device);
    let off_device = Arc::clone(&device);

    // Define the /on endpoint
    let on_route = warp::path("on")
        .and(warp::get())
        .map(move || {
            let device = Arc::clone(&on_device);
            tokio::spawn(async move {
                turn_on_device(&device).await;
            });
            warp::reply::with_status("Device turned on", warp::http::StatusCode::OK)
        });

    // Define the /off endpoint
    let off_route = warp::path("off")
        .and(warp::get())
        .map(move || {
            let device = Arc::clone(&off_device);
            tokio::spawn(async move {
                turn_off_device(&device).await;
            });
            warp::reply::with_status("Device turned off", warp::http::StatusCode::OK)
        });

    let routes = on_route.or(off_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_device() -> Result<tapo::PlugEnergyMonitoringHandler, Box<dyn std::error::Error>> {
    let username = env::var("TAPO_USERNAME").expect("TAPO_USERNAME not set");
    let password = env::var("TAPO_PASSWORD").expect("TAPO_PASSWORD not set");
    let device_ip = env::var("DEVICE_IP").expect("DEVICE_IP not set");

    let client = ApiClient::new(&username, &password);
    let device = client.p110(&device_ip).await?;
    Ok(device)
}

async fn turn_on_device(device: &PlugEnergyMonitoringHandler) {
    device.on().await.expect("Failed to turn on the device");
    println!("Device turned on");
}

async fn turn_off_device(device: &PlugEnergyMonitoringHandler) {
    device.off().await.expect("Failed to turn off the device");
    println!("Device turned off");
}

