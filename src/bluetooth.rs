use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use std::error::Error;

pub async fn scan() -> Result<(), Box<dyn Error>> {
    let manager: Manager = Manager::new().await?;

    let adapter: btleplug::platform::Adapter = manager.adapters().await?.remove(0);

    adapter.start_scan(ScanFilter::default()).await?;

    println!("\x1b[34m[BLUESNIFF]\x1b[0m Scanning for Bluetooth devices...");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let peripherals: Vec<btleplug::platform::Peripheral> = adapter.peripherals().await?;

    for peripheral in peripherals {
        let properties: Option<btleplug::api::PeripheralProperties> =
            peripheral.properties().await?;
        let addr: btleplug::api::BDAddr = peripheral.address();
        let name: String = properties
            .as_ref()
            .and_then(|p| p.local_name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        println!(
            "\x1b[34m[BLUESNIFF]\x1b[0m Address: \x1b[33m{}\x1b[0m, Name: \x1b[36m{}\x1b[0m",
            addr, name
        );
    }

    Ok(())
}
