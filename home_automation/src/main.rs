use std::env;
use tapo::ApiClient;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let username = env::var("TAPO_USERNAME").expect("TAPO_USERNAME not set");
    let password = env::var("TAPO_PASSWORD").expect("TAPO_PASSWORD not set");
    let device_ip = env::var("DEVICE_IP").expect("DEVICE_IP not set");

    let client = ApiClient::new(&username, &password);
    let device = client.p110(&device_ip).await.expect("Failed to connect to device");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <on|off>", args[0]);
        return;
    }

    match args[1].as_str() {
        "on" => {
            device.on().await.expect("Failed to turn on the device");
            println!("Device turned on");
        }
        "off" => {
            device.off().await.expect("Failed to turn off the device");
            println!("Device turned off");
        }
        _ => eprintln!("Invalid command. Use 'on' or 'off'."),
    }
}