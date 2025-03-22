use ed25519_dalek::{SigningKey, Signer};
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::io::vfs::BlockingStdIo;
use log::info;
use rand_core::OsRng;
use bs58;
use std::io::{Read, Write};

/// Loads an existing Solana keypair from NVS or generates and saves a new one.
fn load_or_generate_key(nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<SigningKey> {
    let key_name = "solana_key";
    let mut key_bytes = [0u8; 32];

    match nvs.get_raw(key_name, &mut key_bytes)? {
        Some(_) => {
            info!("Loaded existing key from NVS");
            Ok(SigningKey::from_bytes(&key_bytes))
        }
        None => {
            info!("No key found, generating new keypair...");
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let key_bytes = signing_key.to_bytes();
            nvs.set_raw(key_name, &key_bytes)?;
            info!("Saved new key to NVS");
            Ok(signing_key)
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize the logger for debugging output
    EspLogger::initialize_default();

    // Take ownership of peripherals and NVS partition
    let peripherals = Peripherals::take().unwrap();
    let nvs_partition = EspDefaultNvsPartition::take()?;
    let mut nvs = EspNvs::new(nvs_partition, "solana_signer", true)?;

    // Load or generate the Solana signing key
    let signing_key = load_or_generate_key(&mut nvs)?;
    let verifying_key = signing_key.verifying_key();
    let wallet_address = bs58::encode(verifying_key.to_bytes()).into_string();

    // Log the Solana wallet address
    info!("Solana Wallet Address: {}", wallet_address);
    info!("Ready to sign messages! Enter a message via serial (end with newline).");

    // Configure UART0 for serial communication (TX: GPIO1, RX: GPIO3)
    let uart = UartDriver::new(
        peripherals.uart0,
        peripherals.pins.gpio1, // TX pin
        peripherals.pins.gpio3, // RX pin
        Option::<AnyIOPin>::None, // No CTS
        Option::<AnyIOPin>::None, // No RTS
        &Default::default(), // Default UART settings (115200 baud, 8N1)
    )?;

    // Set up blocking I/O for easy serial interaction
    let _blocking_io = BlockingStdIo::uart(uart)?;

    // Main loop: Prompt for messages, sign them, and display signatures
    loop {
        // Display prompt and ensure it appears on the serial console
        print!("Enter message to sign: ");
        std::io::stdout().flush()?;

        // Read the user's input from the serial console
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        // Clean up the input by trimming whitespace
        let message = buffer.trim();
        if message.is_empty() {
            info!("Empty message, try again.");
        } else {
            // Sign the message and log the result
            info!("Signing message: {}", message);
            let signature = signing_key.sign(message.as_bytes());
            info!("Signature: {:?}", signature.to_bytes());
        }
    }
}