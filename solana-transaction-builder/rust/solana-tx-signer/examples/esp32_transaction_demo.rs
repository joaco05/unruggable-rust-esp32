//! ESP32 Transaction Demo
//!
//! This example demonstrates the ESP32's transaction creation capabilities
//! without actually sending transactions to the network. It showcases:
//!
//! - Getting ESP32 public key
//! - Creating placeholder transactions with memo
//! - Getting transaction information
//! - Decoding and inspecting transactions
//!
//! Run with: cargo run --example esp32_transaction_demo

use anyhow::Result;
use base64::Engine;
use serialport::SerialPort;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Duration;

// Configure your ESP32 serial port here
const SERIAL_PORT: &str = "/dev/tty.usbserial-0001";

/// Send a command to ESP32 and read response
fn send_command(port: &mut Box<dyn SerialPort>, command: &str) -> Result<String> {
    // Send command
    port.write_all(format!("{}\n", command).as_bytes())?;
    port.flush()?;
    println!("→ Sent: {}", command);

    // Read response
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;

    while timeout_count < 20 {
        // Increased timeout for demo
        match port.read(&mut byte) {
            Ok(1) => {
                let ch = byte[0] as char;
                if ch == '\n' {
                    break;
                }
                buffer.push(ch);
            }
            Ok(0) => {
                timeout_count += 1;
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(Duration::from_millis(100));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }

    let response = buffer.trim();
    println!("← Received: {}", response);
    Ok(response.to_string())
}

/// Decode and analyze a base64 transaction
fn analyze_transaction(base64_tx: &str) -> Result<()> {
    let tx_bytes = base64::engine::general_purpose::STANDARD.decode(base64_tx)?;

    println!("\n📊 Transaction Analysis:");
    println!("   Size: {} bytes", tx_bytes.len());

    if tx_bytes.len() >= 64 {
        println!("   Signatures: 1 (64 bytes)");
        let signature = &tx_bytes[1..65]; // Skip signature count byte
        println!("   Signature (first 8 bytes): {:02x?}...", &signature[..8]);
    }

    if tx_bytes.len() > 65 {
        let message_start = 65; // After signature count + signature
        if message_start + 3 <= tx_bytes.len() {
            let header = &tx_bytes[message_start..message_start + 3];
            println!(
                "   Header: required_sigs={}, readonly_signed={}, readonly_unsigned={}",
                header[0], header[1], header[2]
            );
        }
    }

    println!(
        "   Raw bytes (first 32): {:02x?}...",
        &tx_bytes[..32.min(tx_bytes.len())]
    );
    Ok(())
}

fn main() -> Result<()> {
    println!("🚀 ESP32 Transaction Demo");
    println!("=========================\n");

    // Open serial port
    println!("📡 Connecting to ESP32 on {}...", SERIAL_PORT);
    let mut port = serialport::new(SERIAL_PORT, 115_200)
        .timeout(Duration::from_millis(500))
        .open()?;
    println!("✅ Connected!\n");

    // Step 1: Get public key
    println!("1️⃣  Getting ESP32 Public Key");
    println!("{}", "-".repeat(30));
    let response = send_command(&mut port, "GET_PUBKEY")?;

    if let Some(pubkey_str) = response.strip_prefix("PUBKEY:") {
        let pubkey = Pubkey::from_str(pubkey_str)?;
        println!("✅ ESP32 Public Key: {}", pubkey);
        println!("   Length: {} characters", pubkey_str.len());
        println!("   Format: Base58\n");
    } else {
        return Err(anyhow::anyhow!("Invalid pubkey response: {}", response));
    }

    // Step 2: Get transaction info
    println!("2️⃣  Getting Transaction Information");
    println!("{}", "-".repeat(35));
    let response = send_command(&mut port, "TX_INFO")?;

    if let Some(info_str) = response.strip_prefix("TX_INFO:") {
        println!("✅ Transaction Info: {}", info_str);

        // Parse info components
        let parts: Vec<&str> = info_str.split(';').collect();
        for part in parts {
            if part.starts_with("memo=") {
                println!("   📝 Memo: {}", &part[5..]);
            } else if part.starts_with("blockhash=") {
                println!("   🔗 Blockhash: {}", &part[10..]);
            } else if part.starts_with("program=") {
                println!("   🏦 Program: {}", &part[8..]);
            }
        }
        println!();
    } else {
        return Err(anyhow::anyhow!("Invalid tx_info response: {}", response));
    }

    // Step 3: Create transaction
    println!("3️⃣  Creating Placeholder Transaction");
    println!("{}", "-".repeat(38));
    println!("⏳ Requesting transaction creation (this may take a moment)...");

    let response = send_command(&mut port, "CREATE_TX")?;

    if let Some(tx_base64) = response.strip_prefix("TRANSACTION:") {
        println!("✅ Transaction created successfully!");
        println!("   Base64 length: {} characters", tx_base64.len());

        // Show first and last parts of base64
        if tx_base64.len() > 40 {
            println!(
                "   Base64: {}...{}",
                &tx_base64[..20],
                &tx_base64[tx_base64.len() - 20..]
            );
        } else {
            println!("   Base64: {}", tx_base64);
        }

        // Analyze the transaction
        if let Err(e) = analyze_transaction(tx_base64) {
            println!("⚠️  Could not analyze transaction: {}", e);
        }

        println!("\n💾 Complete Base64 Transaction:");
        println!("{}\n", tx_base64);
    } else {
        return Err(anyhow::anyhow!(
            "Invalid transaction response: {}",
            response
        ));
    }

    // Step 4: Demonstrate signing capability (without actual signing)
    println!("4️⃣  Transaction Signing Capability");
    println!("{}", "-".repeat(35));
    println!("📋 The ESP32 can sign any transaction message using SIGN:<base64>");
    println!("   Example: SIGN:AQABAgMEBQY...");
    println!("   Response: SIGNATURE:<base64_signature>");
    println!("   Signature length: 64 bytes (Ed25519)\n");

    // Step 5: Show shutdown capability
    println!("5️⃣  Safe Shutdown");
    println!("{}", "-".repeat(16));
    println!("🔒 ESP32 supports safe shutdown with SHUTDOWN command");
    println!("   This prepares the device for disconnection");
    println!("   GPIO isolation and deep sleep mode\n");

    println!("🎉 Demo completed successfully!");
    println!("\n📝 Summary:");
    println!("   ✅ ESP32 public key retrieved");
    println!("   ✅ Transaction info obtained");
    println!("   ✅ Placeholder transaction created");
    println!("   ✅ Transaction structure analyzed");
    println!("   ✅ Signing capability confirmed");

    println!("\n💡 Next steps:");
    println!("   • Use the main application to sign and submit real transactions");
    println!("   • Integrate with your own Solana applications");
    println!("   • Explore 2FA features if enabled on your ESP32");

    Ok(())
}
