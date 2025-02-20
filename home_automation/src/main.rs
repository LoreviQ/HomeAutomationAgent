use std::env;
use tapo::ApiClient;
use tokio;
use warp::Filter;

async fn connect_device() -> tapo::LightHandler {
    dotenv::dotenv().ok();

    let username = env::var("TAPO_USERNAME").expect("TAPO_USERNAME not set");
    let password = env::var("TAPO_PASSWORD").expect("TAPO_PASSWORD not set");
    let device_ip = env::var("DEVICE_IP").expect("DEVICE_IP not set");

    let client = ApiClient::new(&username, &password);
    client
        .l510(&device_ip)
        .await
        .expect("Failed to connect to device")
}

#[tokio::main]
async fn main() {
    // Define the /on endpoint
    let on_route = warp::path("on").and(warp::get()).map(move || {
        tokio::spawn(async move {
            turn_on().await;
        });
        warp::reply::with_status("Device turned on", warp::http::StatusCode::OK)
    });

    // Define the /off endpoint
    let off_route = warp::path("off").and(warp::get()).map(move || {
        tokio::spawn(async move {
            turn_off().await;
        });
        warp::reply::with_status("Device turned off", warp::http::StatusCode::OK)
    });

    // Define the /brightness/{int} endpoint
    let brightness_route =
        warp::path!("brightness" / u8)
            .and(warp::get())
            .map(move |brightness: u8| {
                tokio::spawn(async move {
                    let brightness = if brightness > 100 { 100 } else { brightness };
                    set_brightness(brightness).await;
                });
                warp::reply::with_status(
                    format!("Brightness set to {}", brightness),
                    warp::http::StatusCode::OK,
                )
            });

    // Define the /info endpoint
    let info_route = warp::path("info")
        .and(warp::get())
        .and_then(get_device_info);

    let routes = on_route.or(off_route).or(brightness_route).or(info_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn turn_on() {
    let device = connect_device().await;
    device.on().await.expect("Failed to turn on the device");
    println!("Device turned on");
}

async fn turn_off() {
    let device = connect_device().await;
    device.off().await.expect("Failed to turn off the device");
    println!("Device turned off");
}

async fn set_brightness(brightness: u8) {
    let device = connect_device().await;
    device
        .set_brightness(brightness)
        .await
        .expect("Failed to set brightness");
    println!("Brightness set to {}", brightness);
}

async fn get_device_info() -> Result<impl warp::Reply, warp::Rejection> {
    let device = connect_device().await;
    let info = device
        .get_device_info()
        .await
        .expect("Failed to get device info");
    Ok(warp::reply::json(&info))
}
