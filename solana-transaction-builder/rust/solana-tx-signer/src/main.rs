use anyhow::Result;
use base64::Engine;
use serialport::SerialPort;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::{Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Signature,
    system_instruction,
    transaction::VersionedTransaction,
};
use std::str::FromStr;
<<<<<<< HEAD

=======
use base64::Engine;
use anyhow::Result;
>>>>>>> 3ed93ca357f4000782500396077be8e4845fe976
// Constants for serial port, RPC URL, recipient public key, and lamports to send
// FIXME: Change this to the correct serial port for your system.
const SERIAL_PORT: &str = "/dev/ttyUSB0";
const RPC_URL: &str = "https://api.devnet.solana.com";
const RECIPIENT_PUBLIC_KEY: &str = "aQQjEjpLuDGq7f7dHC2uqaQt5QWcdYFgvpro74V66hD";
const LAMPORTS_TO_SEND: u64 = 2_000_000;

/// Creates a placeholder transaction with memo on the ESP32 and returns the base64-encoded transaction
fn create_esp32_transaction(port: &mut Box<dyn SerialPort>) -> Result<String> {
    // Send "CREATE_TX" with a newline as expected by ESP32
    port.write_all("CREATE_TX\n".as_bytes())?;
    port.flush()?;
    println!("Requested transaction creation from ESP32");

    // Read the response until newline
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;
    while timeout_count < 10 {
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
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }
    let response = buffer.trim();
    // Check for the expected "TRANSACTION:" prefix and extract the base64 transaction
    if response.starts_with("TRANSACTION:") {
        let transaction_str = &response[12..]; // Skip "TRANSACTION:"
        println!("Received ESP32 transaction: {}", transaction_str);
        Ok(transaction_str.to_string())
    } else {
        Err(anyhow::anyhow!("Invalid response from ESP32: {}", response))
    }
}

/// Gets transaction information from the ESP32
fn get_esp32_transaction_info(port: &mut Box<dyn SerialPort>) -> Result<String> {
    // Send "TX_INFO" with a newline as expected by ESP32
    port.write_all("TX_INFO\n".as_bytes())?;
    port.flush()?;
    println!("Requested transaction info from ESP32");

    // Read the response until newline
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;
    while timeout_count < 10 {
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
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }
    let response = buffer.trim();
    // Check for the expected "TX_INFO:" prefix
    if response.starts_with("TX_INFO:") {
        let info_str = &response[8..]; // Skip "TX_INFO:"
        println!("Received ESP32 transaction info: {}", info_str);
        Ok(info_str.to_string())
    } else {
        Err(anyhow::anyhow!("Invalid response from ESP32: {}", response))
    }
}

/// Retrieves the public key from the ESP32 board via serial communication
fn get_esp32_public_key(port: &mut Box<dyn SerialPort>) -> Result<Pubkey> {
    // Send "GET_PUBKEY" with a newline as expected by ESP32
    port.write_all("GET_PUBKEY\n".as_bytes())?;
    port.flush()?;
    println!("Requested public key from ESP32");

    // Read the response until newline
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;
    while timeout_count < 10 {
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
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }
    let response = buffer.trim();
    // Check for the expected "PUBKEY:" prefix and extract the base58 public key
    if response.starts_with("PUBKEY:") {
        let pubkey_str = &response[7..]; // Skip "PUBKEY:"
        println!("Received ESP32 public key: {}", pubkey_str);
        Pubkey::from_str(pubkey_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse public key: {}", e))
    } else {
        Err(anyhow::anyhow!("Invalid response from ESP32: {}", response))
    }
}

/// Sends the transaction message to the ESP32 and retrieves the signature
fn send_to_esp32_and_get_signature(
    port: &mut Box<dyn SerialPort>,
    base64_message: &str,
) -> Result<String> {
    let sign_command = format!("SIGN:{}", base64_message);
    port.write_all(sign_command.as_bytes())?;
    port.write_all(b"\n")?;
    port.flush()?;
    println!("Sent to ESP32: {}", sign_command);

    // Clear the input buffer to ensure we read the new response
    port.clear(serialport::ClearBuffer::Input)?;

    // Rest of your function remains unchanged
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;


    while timeout_count < 10 {
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
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }
    let response = buffer.trim();
    if response.starts_with("SIGNATURE:") {
        let base64_signature = &response[10..];
        println!("Received signature from ESP32: {}", base64_signature);
        Ok(base64_signature.to_string())
    } else {
        Err(anyhow::anyhow!("Invalid response from ESP32: {}", response))
    }
}

/// Sends the SHUTDOWN command to the ESP32 to prepare it for safe disconnection
fn shutdown_esp32(port: &mut Box<dyn SerialPort>) -> Result<()> {
    // Send "SHUTDOWN" with a newline as expected by ESP32
    port.write_all("SHUTDOWN\n".as_bytes())?;
    port.flush()?;
    println!("Sent SHUTDOWN command to ESP32");

    // Read the confirmation response until newline (similar to other reads)
    let mut buffer = String::new();
    let mut byte = [0u8; 1];
    let mut timeout_count = 0;
    while timeout_count < 10 {
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
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => {
                timeout_count += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Ok(n) => unreachable!("Unexpected read size: {}", n),
        }
    }
    let response = buffer.trim();
    if response == "SHUTDOWN_OK" {
        println!("Received shutdown confirmation from ESP32: {}", response);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Invalid or no shutdown confirmation from ESP32: {}",
            response
        ))
    }
}

fn main() -> Result<()> {
    println!("=== ESP32 Solana Transaction Builder ===");

    // Initialize the Solana RPC client
    let client = RpcClient::new(RPC_URL.to_string());

    // Open the serial port to communicate with the ESP32
    let mut port = match serialport::new(SERIAL_PORT, 115_200)
        .timeout(std::time::Duration::from_secs(1))
        .open() {
            Ok(port) => port,
            Err(e) => {
                eprintln!("Failed to open serial port '{}': {}", SERIAL_PORT, e);
                return Err(e.into());
            }
        };

    println!("\n1. Getting ESP32 public key...");
    // Get the ESP32 public key, which will be the fee payer and signer
    let esp32_pubkey = get_esp32_public_key(&mut port)?;

    println!("\n2. Getting transaction info from ESP32...");
    // Get transaction information from ESP32
    let _tx_info = get_esp32_transaction_info(&mut port)?;

    println!("\n3. Creating placeholder transaction on ESP32...");
    // Create a placeholder transaction with memo on ESP32
    let base64_transaction = create_esp32_transaction(&mut port)?;

    // Decode the transaction to inspect it
    let transaction_bytes =
        base64::engine::general_purpose::STANDARD.decode(&base64_transaction)?;
    println!(
        "ESP32 created transaction ({} bytes)",
        transaction_bytes.len()
    );

    // For demonstration, we can also create a traditional transfer transaction
    println!("\n4. Creating traditional transfer transaction...");

    // Parse the recipient public key from the constant string
    let recipient_pubkey = Pubkey::from_str(RECIPIENT_PUBLIC_KEY)?;

    // Fetch the latest blockhash with finalized commitment
    let (recent_blockhash, _last_valid_slot) =
        client.get_latest_blockhash_with_commitment(CommitmentConfig::finalized())?;

    // Create a transfer instruction
    let instruction =
        system_instruction::transfer(&esp32_pubkey, &recipient_pubkey, LAMPORTS_TO_SEND);
    let mut message = Message::new(&[instruction], Some(&esp32_pubkey));
    message.recent_blockhash = recent_blockhash;

    // Create a VersionedTransaction with the message and an empty signature slot
    let mut transaction = VersionedTransaction {
        signatures: vec![Signature::default(); message.header.num_required_signatures as usize],
        message: VersionedMessage::Legacy(message),
    };

    // Print the number of signatures expected for verification
    println!(
        "Number of signatures expected: {}",
        transaction.message.header().num_required_signatures
    );

    // Serialize the transaction message to bytes for signing
    let message_bytes = transaction.message.serialize();
    let base64_message_to_sign = base64::engine::general_purpose::STANDARD.encode(&message_bytes);
    println!(
        "Serialized Transaction Message (Base64): {}",
        base64_message_to_sign
    );

    println!("\n5. Signing transaction with ESP32...");
    // Send the serialized message to the ESP32 and get the base64-encoded signature
    let base64_signature = send_to_esp32_and_get_signature(&mut port, &base64_message_to_sign)?;

    // Decode the base64 signature into bytes and convert to a Solana Signature
    let signature_bytes = base64::engine::general_purpose::STANDARD.decode(&base64_signature)?;
    let signature = Signature::try_from(signature_bytes.as_slice())?;

    // Verify that the transaction expects exactly one signature
    if transaction.signatures.len() != 1 {
        return Err(anyhow::anyhow!(
            "Expected 1 signature slot, found {}",
            transaction.signatures.len()
        ));
    }

    // Assign the signature received from ESP32 to the transaction
    transaction.signatures[0] = signature;

    println!("\n6. Sending transaction to Solana network...");
    // Send the signed transaction to the Solana network
    let signature = client.send_transaction(&transaction)?;
    println!("Transaction sent with signature: {}", signature);

    // Confirm the transaction has been processed on the network
    client.confirm_transaction(&signature)?;
    println!("Transaction confirmed");

    println!("\n7. Shutting down ESP32...");
    // Shutdown the ESP32 after transaction confirmation
    shutdown_esp32(&mut port)?;

    println!("\n=== Transaction process completed successfully! ===");
    Ok(())
}
