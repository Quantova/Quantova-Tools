// Quantum-Resistant Digital Signature Example using Falcon512
//
// This code demonstrates the use of the post-quantum cryptographic algorithm Falcon512.
// Falcon is a lattice-based digital signature scheme designed for efficiency and compact signatures.
// The program follows these steps:
// 1. Generate a public-secret key pair
// 2. Sign a message using the private key
// 3. Verify the signature using the public key

// Import necessary cryptographic libraries from pqcrypto-falcon and pqcrypto-traits
use pqcrypto_falcon::falcon512;
use pqcrypto_traits::sign::{PublicKey, SignedMessage};

fn main() {
    // === Step 1: Generate a Key Pair ===
    // The key pair consists of:
    // - Public Key (pk): Used for signature verification
    // - Secret Key (sk): Used for signing messages
    let (pk, sk) = falcon512::keypair();
    println!("Public Key: {:?}", pk.as_bytes());

    // === Step 2: Define the Message ===
    // This is the message that will be digitally signed.
    let message = b"Quantum Resistant Blockchain Message";

    // === Step 3: Sign the Message ===
    // The private key (sk) is used to create a digital signature for the message.
    // The output is a SignedMessage struct that includes both the message and its signature.
    let signed_message = falcon512::sign(message, &sk);
    println!("Signed Message Length: {} bytes", signed_message.as_bytes().len());

    // === Step 4: Verify the Signed Message ===
    // Verification checks if the signature is valid using the corresponding public key.
    // If verification succeeds, it returns the original message.
    match falcon512::open(&signed_message, &pk) {
        Ok(valid_message) => {
            println!("Verified successfully: {:?}", String::from_utf8_lossy(&valid_message));
        },
        Err(_) => {
            println!("Signature verification failed!");
        }
    }
}