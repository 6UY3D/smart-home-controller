use embedded_hal::digital::v2::OutputPin;
use log::{info, error};

pub struct DisplayManager<PIN> {
    pin: PIN,
}

impl<PIN: OutputPin> DisplayManager<PIN> {
    pub fn new(pin: PIN) -> Self {
        DisplayManager { pin }
    }

    pub fn update_display(&mut self, message: &str) {
        info!("Updating display: {}", message);
        if let Err(e) = self.pin.set_high() {
            error!("Failed to update display: {:?}", e);
        }
    }
}
