use ed25519_dalek::{SigningKey, Signer, VerifyingKey};
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::sys::ESP_ERR_TIMEOUT;
use rand_core::OsRng;
use bs58;
use base64;
use base64::Engine;

fn load_or_generate_key(nvs: &mut EspNvs<NvsDefault>) -> anyhow::Result<SigningKey> {
    let key_name = "solana_key";
    let mut key_bytes = [0u8; 32];
    match nvs.get_raw(key_name, &mut key_bytes)? {
        Some(_) => Ok(SigningKey::from_bytes(&key_bytes)),
        None => {
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let key_bytes = signing_key.to_bytes();
            nvs.set_raw(key_name, &key_bytes)?;
            Ok(signing_key)
        }
    }
}

fn send_response(uart: &mut UartDriver, response: &str) -> anyhow::Result<()> {
    let response_with_newline = response.to_string() + "\n";
    let data = response_with_newline.as_bytes();
    let mut written = 0;
    while written < data.len() {
        written += uart.write(&data[written..])?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let nvs_partition = EspDefaultNvsPartition::take()?;
    let mut nvs = EspNvs::new(nvs_partition, "solana_signer", true)?;
    let signing_key = load_or_generate_key(&mut nvs)?;
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    let pubkey_bytes = verifying_key.to_bytes();
    let pubkey_base58 = bs58::encode(pubkey_bytes).into_string();

    let mut uart = UartDriver::new(
        peripherals.uart0,
        peripherals.pins.gpio1,
        peripherals.pins.gpio3,
        Option::<esp_idf_svc::hal::gpio::AnyIOPin>::None,
        Option::<esp_idf_svc::hal::gpio::AnyIOPin>::None,
        &Default::default(),
    )?;

    // Configure BOOT button (GPIO 0) as input with pull-up
    let mut button = PinDriver::input(peripherals.pins.gpio0)?;
    button.set_pull(Pull::Up)?;

    let mut buffer = String::new();
    loop {
        let mut byte = [0u8; 1];
        match uart.read(&mut byte, 1000) {
            Ok(1) => {
                let ch = byte[0] as char;
                if ch == '\n' {
                    let input = buffer.trim();
                    if input == "GET_PUBKEY" {
                        send_response(&mut uart, &pubkey_base58)?;
                    } else if !input.is_empty() {
                        match base64::engine::general_purpose::STANDARD.decode(input) {
                            Ok(message_bytes) => {
                                // Wait for the BOOT button to be pressed
                                while !button.is_low() {
                                    esp_idf_svc::hal::delay::FreeRtos::delay_ms(100); // Avoid busy-waiting
                                }
                                let signature = signing_key.sign(&message_bytes);
                                let signature_bytes = signature.to_bytes();
                                let base64_signature = base64::engine::general_purpose::STANDARD.encode(&signature_bytes);
                                send_response(&mut uart, &base64_signature)?;
                            }
                            Err(_) => {
                                // Ignore decoding errors silently
                            }
                        }
                    }
                    buffer.clear();
                } else {
                    buffer.push(ch);
                }
            }
            Ok(0) => {}
            Ok(n) => unreachable!("Unexpected read size: {}", n),
            Err(e) => {
                if e.code() != ESP_ERR_TIMEOUT {
                    // Handle non-timeout errors if needed
                }
            }
        }
    }
}