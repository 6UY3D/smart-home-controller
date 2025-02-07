mod ble;
mod audio;
mod display;
mod server;
mod utils;

use ble::BLEManager;
use audio::AudioManager;
use display::DisplayManager;
use server::ServerManager;
use utils::{load_config, Config};
use esp_idf_sys as _; // If using the Espressif IDF
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use log::info;

fn main() {
    esp_idf_sys::link_patches(); // Bind Rust to ESP-IDF
    env_logger::init(); // Initialize logger

    let config = load_config("config.toml").expect("Failed to load configuration");

    let peripherals = Peripherals::take().unwrap();
    let mut display_pin = PinDriver::output(peripherals.pins.gpio0).unwrap();

    let ble_manager = BLEManager::new().expect("Failed to initialize BLE manager");
    let audio_manager = AudioManager::new().expect("Failed to initialize audio manager");
    let mut display_manager = DisplayManager::new(display_pin);
    let server_manager = ServerManager::new(config.server.endpoint);

    info!("Initializing smart home controller...");

    ble_manager.scan_and_connect();
    audio_manager.play_audio(b"Audio data");
    display_manager.update_display("Hello, World!");
    server_manager.send_data(&serde_json::json!({"status": "online"}));

    info!("Smart home controller initialized.");
}
