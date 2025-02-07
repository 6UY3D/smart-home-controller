# Utils Module

## Purpose
The `utils` module provides utility functions for loading configuration files and other helper functions.

## Functions

### `load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>>`
- **Description**: Loads the configuration from the specified TOML file.
- **Parameters**:
  - `path`: The path to the configuration file.
- **Returns**: A `Result` containing a `Config` instance on success, or an error on failure.
- **Logs**: Logs the successful loading of the configuration.

## Structs

### `Config`
- **Description**: Represents the configuration loaded from the TOML file.
- **Fields**:
  - `server`: Server configuration.
  - `ble`: BLE configuration.
  - `audio`: Audio configuration.

### `ServerConfig`
- **Description**: Represents the server configuration.
- **Fields**:
  - `endpoint`: The server endpoint URL.

### `BLEConfig`
- **Description**: Represents the BLE configuration.
- **Fields**:
  - `scan_interval`: The interval between BLE scans, in seconds.

### `AudioConfig`
- **Description**: Represents the audio configuration.
- **Fields**:
  - `sample_rate`: The audio sample rate, in Hz.
