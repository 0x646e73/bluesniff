# bluesniff

**bluesniff** is a simple Bluetooth device scanner written in Rust. It scans for nearby Bluetooth devices and lists their MAC addresses and names. This project uses the `btleplug` crate to handle Bluetooth operations and the `tokio` crate for asynchronous functionality.

## Features

- Scans for Bluetooth devices in the vicinity.
- Displays the MAC addresses and names of detected devices.
- Built with Rust's asynchronous capabilities for efficient operation.

## Prerequisites

To build and run **bluesniff**, you need:

- [Rust](https://www.rust-lang.org/) installed on your machine.
- A Bluetooth adapter that supports BLE (Bluetooth Low Energy).
- The necessary permissions to access Bluetooth devices.

## Installation


You can install **bluesniff** directly from `crates.io`:

```bash
cargo install bluesniff
```

Alternatively, if you want to build from source, clone the repository:

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/bluesniff.git
   cd bluesniff
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the scanner with:

   ```bash
   cargo run
   ```

   Note: You may need to run the command with `sudo` depending on your system configuration.

## Usage

Upon running **bluesniff**, the program will start scanning for Bluetooth devices for a specified duration (5 seconds). It will display the MAC addresses and names of any detected devices.

Example output:
```
[BLUESNIFF] Initializing scan...
[BLUESNIFF] Scanning for Bluetooth devices...
[BLUESNIFF] Address: XX:XX:XX:XX:XX:XX, Name: Device Name
[BLUESNIFF] Done scanning...
```

## Contributing

Contributions are welcome! If you would like to improve **bluesniff**, feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [btleplug](https://crates.io/crates/btleplug) - for providing Bluetooth support in Rust.
- [tokio](https://tokio.rs/) - for asynchronous programming in Rust.
