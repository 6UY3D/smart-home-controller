use esp_idf_hal::i2s::I2SDriver;
use log::{info, error};

pub struct AudioManager {
    i2s: I2SDriver,
}

impl AudioManager {
    pub fn new() -> Result<Self, &'static str> {
        let i2s = I2SDriver::new(/* configuration */)
            .map_err(|_| "Failed to initialize I2S driver")?;
        Ok(AudioManager { i2s })
    }

    pub fn play_audio(&self, data: &[u8]) {
        info!("Playing audio...");
        if let Err(e) = self.i2s.write(data) {
            error!("Failed to play audio: {:?}", e);
        }
    }
}
