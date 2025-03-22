const { Connection, Transaction, SystemProgram, PublicKey } = require('@solana/web3.js');

// Configuration
const ESP32_PUBLIC_KEY = new PublicKey("HY2WuCKnUW9a41AXboDVVBnEX4DUjg9GqZxyvf1ETpK2"); // Replace with a valid public key
const RECIPIENT_PUBLIC_KEY = new PublicKey("6tBou5MHL5aWpDy6cgf3wiwGGK2mR8qs68ujtpaoWrf2"); // Ensure this is valid too
const LAMPORTS_TO_SEND = 1000000; // 0.001 SOL

async function createUnsignedTransaction() {
  // Connect to the Solana cluster
  const connection = new Connection('https://special-blue-fog.solana-mainnet.quiknode.pro/d009d548b4b9dd9f062a8124a868fb915937976c/', 'confirmed');

  // Get the latest blockhash (required for a valid transaction)
  const { blockhash } = await connection.getLatestBlockhash();

  // Create a new transaction
  const transaction = new Transaction({
    recentBlockhash: blockhash,
    feePayer: ESP32_PUBLIC_KEY, // The ESP32 will pay the fee (and sign later)
  }).add(
    SystemProgram.transfer({
      fromPubkey: ESP32_PUBLIC_KEY,
      toPubkey: RECIPIENT_PUBLIC_KEY,
      lamports: LAMPORTS_TO_SEND,
    })
  );

  // Serialize the transaction message (this is what needs to be signed)
  const messageToSign = transaction.serializeMessage();
  const base64Message = messageToSign.toString('base64');

  console.log('Serialized Transaction Message (Base64):', base64Message);
  return base64Message;
}

async function main() {
  try {
    const serializedMessage = await createUnsignedTransaction();
    console.log('Unsigned transaction created successfully!');
  } catch (error) {
    console.error('Error:', error);
  }
}

main();