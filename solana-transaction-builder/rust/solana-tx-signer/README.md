# ESP32 Solana Transaction Builder

A comprehensive tool for creating, signing, and submitting Solana transactions using an ESP32 hardware signer. This client communicates with the ESP32 over serial to leverage its secure key storage and signing capabilities.

## Features

### ESP32 Hardware Signer Integration
- **Secure Key Management**: Private keys stored securely on ESP32 hardware
- **Hardware Signing**: All cryptographic operations performed on ESP32
- **Serial Communication**: Reliable UART-based protocol
- **2FA Support**: Optional TOTP-based two-factor authentication

### Transaction Capabilities
- **CREATE_TX**: Create placeholder transactions with memo on ESP32
- **Traditional Transfers**: Create standard SOL transfer transactions
- **Custom Signing**: Sign any transaction message with ESP32
- **Network Submission**: Submit signed transactions to Solana network

### Supported Commands
1. `GET_PUBKEY` - Retrieve ESP32 public key
2. `CREATE_TX` - Create placeholder transaction with memo
3. `TX_INFO` - Get transaction information
4. `SIGN:` - Sign transaction messages
5. `SHUTDOWN` - Safely shutdown ESP32

## Prerequisites

- Rust 1.70+ with Cargo
- ESP32 device flashed with the companion signer firmware
- USB cable for serial communication
- Access to a Solana RPC endpoint

## Installation

1. Clone the repository:
```bash
git clone <repository_url>
cd unruggable-rust-esp32/solana-transaction-builder/rust/solana-tx-signer
```

2. Install dependencies:
```bash
cargo build
```

## Configuration

Edit the constants in `src/main.rs` to match your setup:

```rust
const SERIAL_PORT: &str = "/dev/tty.usbserial-0001";  // Your ESP32 serial port
const RPC_URL: &str = "api";                          // Your Solana RPC endpoint
const RECIPIENT_PUBLIC_KEY: &str = "6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2";
const LAMPORTS_TO_SEND: u64 = 1_000_000;             // Amount in lamports
```

### Finding Your Serial Port

**macOS/Linux:**
```bash
ls /dev/tty.usbserial-* # macOS
ls /dev/ttyUSB*         # Linux
```

**Windows:**
```
Device Manager > Ports (COM & LPT)
```

## Usage

### Basic Operation

Run the transaction builder:
```bash
cargo run
```

The program will:
1. Connect to ESP32 via serial port
2. Retrieve the ESP32's public key
3. Get transaction information from ESP32
4. Create a placeholder transaction with memo
5. Create a traditional SOL transfer transaction
6. Sign the transfer transaction with ESP32
7. Submit to Solana network
8. Confirm transaction
9. Safely shutdown ESP32

### Example Output

```
=== ESP32 Solana Transaction Builder ===

1. Getting ESP32 public key...
Requested public key from ESP32
Received ESP32 public key: 8K7wYbY1bVq2E3rJ9pF8H4mN5nQ6tS9vX2zC1dA3eG4f

2. Getting transaction info from ESP32...
Requested transaction info from ESP32
Received ESP32 transaction info: memo='Hello from ESP32 Solana Signer!';blockhash=11111111111111111111111111111112;program=MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr

3. Creating placeholder transaction on ESP32...
Requested transaction creation from ESP32
Received ESP32 transaction: eyJ0eXBlIjoidHJhbnNhY3Rpb24i...
ESP32 created transaction (256 bytes)

4. Creating traditional transfer transaction...
Number of signatures expected: 1
Serialized Transaction Message (Base64): AQABAgME...

5. Signing transaction with ESP32...
Sent to ESP32: SIGN:AQABAgME...
Received signature from ESP32: c2lnbmF0dXJl...

6. Sending transaction to Solana network...
Transaction sent with signature: 5j8K2m3N4o5P6q7R8s9T1u2V3w4X5y6Z7a8B9c1D2e3F

Transaction confirmed

7. Shutting down ESP32...
Sent SHUTDOWN command to ESP32
Received shutdown confirmation from ESP32: SHUTDOWN_OK

=== Transaction process completed successfully! ===
```

## ESP32 Transaction Features

### Placeholder Transaction with Memo

The ESP32 can create complete Solana transactions with memo instructions:

- **Memo Text**: "Hello from ESP32 Solana Signer!"
- **Program**: Solana Memo Program (`MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`)
- **Blockhash**: Constant nonce `11111111111111111111111111111112`
- **Signing**: Proper Ed25519 signature with ESP32's private key

### Transaction Structure

The ESP32 creates transactions following Solana's wire format:
1. Signatures section (64 bytes per signature)
2. Message header (3 bytes)
3. Account addresses (32 bytes each)
4. Recent blockhash (32 bytes)
5. Instructions with data

## API Reference

### Core Functions

#### `get_esp32_public_key(port) -> Result<Pubkey>`
Retrieves the public key from the ESP32 device.

#### `create_esp32_transaction(port) -> Result<String>`
Creates a placeholder transaction with memo on the ESP32.

#### `get_esp32_transaction_info(port) -> Result<String>`
Gets information about the ESP32's transaction capabilities.

#### `send_to_esp32_and_get_signature(port, message) -> Result<String>`
Signs a transaction message using the ESP32's private key.

#### `shutdown_esp32(port) -> Result<()>`
Safely shuts down the ESP32 device.

### Serial Protocol

All commands are sent as ASCII strings terminated with `\n`:

| Command | Description | Response Format |
|---------|-------------|-----------------|
| `GET_PUBKEY` | Get public key | `PUBKEY:<base58_pubkey>` |
| `CREATE_TX` | Create transaction | `TRANSACTION:<base64_tx>` |
| `TX_INFO` | Get tx info | `TX_INFO:<info_string>` |
| `SIGN:<base64>` | Sign message | `SIGNATURE:<base64_sig>` |
| `SHUTDOWN` | Shutdown device | `SHUTDOWN_OK` |

## Error Handling

The application includes comprehensive error handling for:
- Serial port connection failures
- ESP32 communication timeouts
- Invalid response formats
- Signature verification failures
- Network transmission errors

Common error patterns:
```rust
Err(anyhow::anyhow!("Invalid response from ESP32: {}", response))
```

## Security Considerations

### Hardware Security
- Private keys never leave the ESP32 device
- All signing operations performed in hardware
- Secure key generation using hardware RNG

### Communication Security
- Serial communication over USB (physical connection required)
- Base64 encoding for data transmission
- Command/response validation

### Network Security
- Transaction confirmation before completion
- RPC endpoint validation
- Signature verification

## Troubleshooting

### Serial Port Issues
```bash
# Check port permissions (Linux)
sudo usermod -a -G dialout $USER
# Log out and back in

# Test port access
ls -la /dev/ttyUSB0
```

### ESP32 Connection
- Ensure ESP32 is flashed with compatible firmware
- Verify baud rate (115200)
- Check USB cable and port
- Monitor ESP32 LED indicators

### Transaction Failures
- Verify sufficient SOL balance for fees
- Check network connectivity
- Confirm recipient address is valid
- Ensure blockhash is recent

## Development

### Building from Source
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Adding Custom Commands

To add new ESP32 commands:
1. Implement the command in ESP32 firmware
2. Add corresponding function in this client
3. Follow the existing serial protocol pattern

Example:
```rust
fn send_custom_command(port: &mut Box<dyn SerialPort>) -> Result<String> {
    port.write_all("CUSTOM_COMMAND\n".as_bytes())?;
    port.flush()?;
    // Handle response...
}
```

## Dependencies

- `solana-sdk` - Solana blockchain SDK
- `solana-client` - RPC client for network communication
- `serialport` - Serial port communication
- `base64` - Base64 encoding/decoding
- `anyhow` - Error handling
- `bs58` - Base58 encoding (Solana addresses)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Related Projects

- [ESP32 Solana Signer Firmware](../../../esp32-solana-signer/) - The companion ESP32 firmware
- [Solana Web3.js](https://solana-labs.github.io/solana-web3.js/) - JavaScript SDK
- [Solana CLI](https://docs.solana.com/cli) - Official command-line tools