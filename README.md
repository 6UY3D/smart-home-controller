# Smart Home Controller

This project implements a smart home controller firmware for the ESP32-S3 using Rust. It includes BLE pairing, real-time audio handling, display output, and server communication.

## Features

- BLE pairing to connect and control smart home devices.
- Real-time audio handling for voice commands or alerts.
- Display output for status updates and user interaction.
- Server communication to integrate with cloud services for remote control and data logging.

## Setup

1. Install Rust and the ESP32 target.
2. Install ESP-IDF using `espup`.
3. Clone this repository and build the project using `cargo build --release`.
4. Flash the firmware to your ESP32-S3 device.

## Configuration

Edit the `config.toml` file to configure server endpoints, BLE scan intervals, and audio settings.

## Usage

- The firmware will scan for BLE devices, handle audio playback, update the display, and communicate with a server for remote control and data logging.
