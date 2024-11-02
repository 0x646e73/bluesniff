mod bluetooth;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("\x1b[34m[BLUESNIFF]\x1b[0m Initializing scan...");
    bluetooth::scan().await?;
    println!("\x1b[34m[BLUESNIFF]\x1b[0m Done scanning...");
    Ok(())
}
