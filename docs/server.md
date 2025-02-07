# Server Module

## Purpose
The `server` module handles communication with a remote server, sending data for remote control and logging.

## Functions

### `new(endpoint: String) -> ServerManager`
- **Description**: Initializes a new `ServerManager` instance with the specified server endpoint.
- **Parameters**:
  - `endpoint`: The URL of the server endpoint.
- **Returns**: A `ServerManager` instance.

### `send_data(&self, data: &serde_json::Value)`
- **Description**: Sends the provided data to the server.
- **Parameters**:
  - `data`: The data to be sent, in JSON format.
- **Logs**: Logs the data being sent and any errors that occur during the process.
