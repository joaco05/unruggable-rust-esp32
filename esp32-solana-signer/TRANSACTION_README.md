# ESP32 Solana Transaction Functionality

This document describes the new transaction creation functionality added to the ESP32 Solana Signer.

## New Commands

### CREATE_TX
Creates a placeholder Solana transaction with a memo instruction.

**Usage:**
```
CREATE_TX
```

**Response:**
```
TRANSACTION:<base64_encoded_transaction>
```

The transaction contains:
- A memo instruction with the text "Hello from ESP32 Solana Signer!"
- Uses a constant nonce as the blockhash: `11111111111111111111111111111112`
- Signed with the device's Ed25519 key
- Targets the Solana memo program: `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`

### TX_INFO
Displays information about the placeholder transaction structure.

**Usage:**
```
TX_INFO
```

**Response:**
```
TX_INFO:memo='Hello from ESP32 Solana Signer!';blockhash=11111111111111111111111111111112;program=MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr
```

## Transaction Structure

The created transaction follows the standard Solana wire format:

1. **Signatures Section:**
   - Number of signatures (1 byte): `0x01`
   - Ed25519 signature (64 bytes)

2. **Message Section:**
   - **Header (3 bytes):**
     - Required signatures: `0x01`
     - Readonly signed accounts: `0x00`
     - Readonly unsigned accounts: `0x01`
   
   - **Account Addresses:**
     - Number of accounts: `0x02`
     - Signer's public key (32 bytes)
     - Memo program ID (32 bytes)
   
   - **Recent Blockhash (32 bytes):**
     - Constant value: `11111111111111111111111111111112`
   
   - **Instructions:**
     - Number of instructions: `0x01`
     - Program ID index: `0x01` (memo program)
     - Number of accounts: `0x01`
     - Account index: `0x00` (signer)
     - Data length: variable
     - Memo text: "Hello from ESP32 Solana Signer!"

## Constants

- **PLACEHOLDER_BLOCKHASH**: `"11111111111111111111111111111112"`
  - A valid base58-encoded 32-byte hash used as a dummy blockhash
  - This makes the transaction deterministic for testing purposes

- **MEMO_PROGRAM_ID**: `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`
  - The official Solana memo program ID
  - Stored as 32-byte array in the code

## LED Indicators

- **CREATE_TX Success**: Triple blink (150ms on/off each)
- **CREATE_TX Error**: Five rapid blinks (100ms on/off each)
- **TX_INFO**: Single short blink (100ms)

## Implementation Notes

The transaction creation is implemented without the full `solana-sdk` dependency to maintain compatibility with ESP32. The transaction structure is manually constructed following the Solana wire format specification.

The transaction is signed using the device's Ed25519 private key, and the signature is computed directly over the raw message bytes (no hashing - Ed25519 handles this internally). Serialization uses Borsh format, which is the standard in the Solana ecosystem.

## Example Usage

```bash
# Get the device's public key
echo "GET_PUBKEY" > /dev/ttyUSB0

# Create a placeholder transaction
echo "CREATE_TX" > /dev/ttyUSB0

# Get transaction info
echo "TX_INFO" > /dev/ttyUSB0
```

The returned base64-encoded transaction can be decoded and analyzed using standard Solana tools or submitted to a Solana cluster (though it will likely fail due to the dummy blockhash).

## Building and Flashing

### Prerequisites
- ESP32 development environment with `esp` toolchain installed
- `espflash` tool for flashing

### Build Instructions
```bash
cd esp32-solana-signer
cargo +esp build
```

### Flash Instructions
```bash
espflash flash target/xtensa-esp32-espidf/debug/esp32-solana-signer --port /dev/tty.usbserial-0001
```
(Replace `/dev/tty.usbserial-0001` with your actual ESP32 serial port)

## Corrections Made

### Key Technical Corrections:
1. **No `no_std`**: ESP-IDF uses the standard library, not `no_std` environment
2. **Borsh over Bincode**: Switched to Borsh serialization which is the standard in Solana ecosystem
3. **Direct Message Signing**: Removed SHA-256 pre-hashing - Ed25519 handles internal hashing automatically
4. **Proper Ed25519 Usage**: Signs raw message bytes directly, as per Solana's specification
5. **Memo Program ID Bytes**: Corrected the byte array for `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`
6. **Embassy Linking Fix**: Removed unnecessary Embassy features from esp-idf-svc to resolve linker errors

### Dependencies Updated:
- Removed: `bincode`, `sha2`
- Added: `borsh` (default-features = false for embedded compatibility)
- Fixed: `esp-idf-svc` features (removed `embassy-time-driver`, `embassy-sync`)

### Signing Process:
- **Before**: SHA-256 hash of message → Ed25519 signature
- **After**: Raw message bytes → Ed25519 signature (correct approach)

### Program ID Correction:
- **Initial**: Incorrect byte array conversion
- **Fixed**: Properly decoded `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` to correct 32-byte array

### Build Fix:
- **Problem**: Embassy executor linking error (`undefined reference to '__pender'`)
- **Solution**: Removed Embassy features from esp-idf-svc dependency that weren't needed

This implementation now correctly follows Solana's transaction signing specification, uses ecosystem-standard serialization, and compiles successfully for ESP32.