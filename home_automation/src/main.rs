use std::env;
use once_cell::sync::Lazy;
use tapo::ApiClient;
use tokio;
use warp::Filter;

// Define global environment variables
static USERNAME: Lazy<String> = Lazy::new(|| {
    env::var("TAPO_USERNAME").expect("TAPO_USERNAME not set")
});

static PASSWORD: Lazy<String> = Lazy::new(|| {
    env::var("TAPO_PASSWORD").expect("TAPO_PASSWORD not set")
});

static DEVICE_IP: Lazy<String> = Lazy::new(|| {
    env::var("DEVICE_IP").expect("DEVICE_IP not set")
});

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Define the /on endpoint
    let on_route = warp::path("on")
        .and(warp::get())
        .map(move || {
            tokio::spawn(async move {
                turn_on().await;
            });
            warp::reply::with_status("Device turned on", warp::http::StatusCode::OK)
        });

    // Define the /off endpoint
    let off_route = warp::path("off")
        .and(warp::get())
        .map(move || {
            tokio::spawn(async move {
                turn_off().await;
            });
            warp::reply::with_status("Device turned off", warp::http::StatusCode::OK)
        });
    
    // Define the /brightness/{int} endpoint
    let brightness_route = warp::path!("brightness" / u8)
        .and(warp::get())
        .map(move |brightness: u8| {
            tokio::spawn(async move {
                let brightness = if brightness > 100 { 100 } else { brightness };
                set_brightness(brightness).await;
            });
            warp::reply::with_status(format!("Brightness set to {}", brightness), warp::http::StatusCode::OK)
        });

    let routes = on_route.or(off_route).or(brightness_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}



async fn turn_on() {
    let client = ApiClient::new(&(*USERNAME), &(*PASSWORD));
    let device = client.l510(&(*DEVICE_IP)).await.expect("Failed to connect to device");
    device.on().await.expect("Failed to turn on the device");
    println!("Device turned on\n");
}

async fn turn_off() {
    let client = ApiClient::new(&(*USERNAME), &(*PASSWORD));
    let device = client.l510(&(*DEVICE_IP)).await.expect("Failed to connect to device");
    device.off().await.expect("Failed to turn off the device");
    println!("Device turned off\n");
}

async fn set_brightness(brightness: u8) {
    let client = ApiClient::new(&(*USERNAME), &(*PASSWORD));
    let device = client.l510(&(*DEVICE_IP)).await.expect("Failed to connect to device");
    device.set_brightness(brightness).await.expect("Failed to set brightness");
    println!("Brightness set to {}\n", brightness);
}
