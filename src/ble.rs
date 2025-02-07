use esp_idf_svc::ble::client::BLEClient;
use embedded_svc::ble::client::BLEClient as _;
use log::{info, error};

pub struct BLEManager {
    client: BLEClient,
}

impl BLEManager {
    pub fn new() -> Result<Self, &'static str> {
        let client = BLEClient::new().map_err(|_| "Failed to create BLE client")?;
        Ok(BLEManager { client })
    }

    pub fn scan_and_connect(&self) {
        info!("Scanning for BLE devices...");
        match self.client.scan() {
            Ok(_) => info!("Scan successful"),
            Err(e) => error!("BLE scan failed: {:?}", e),
        }
    }
}
