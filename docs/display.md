# Display Module

## Purpose
The `display` module manages display output, updating the display with status messages or other information.

## Functions

### `new(pin: PIN) -> DisplayManager<PIN>`
- **Description**: Initializes a new `DisplayManager` instance with the specified GPIO pin.
- **Parameters**:
  - `pin`: The GPIO pin used to control the display.
- **Returns**: A `DisplayManager` instance.

### `update_display(&mut self, message: &str)`
- **Description**: Updates the display with the provided message.
- **Parameters**:
  - `message`: The message to be displayed.
- **Logs**: Logs the message being displayed and any errors that occur during the process.
