// Quantum-Resistant Digital Signature Example using Dilithium3
//
// This code demonstrates the use of the post-quantum cryptographic algorithm Dilithium3.
// Dilithium is a lattice-based digital signature scheme designed to resist quantum attacks.
// The program follows these steps:
// 1. Generate a public-secret key pair
// 2. Sign a message using the private key
// 3. Verify the signature using the public key

// Import necessary cryptographic libraries from pqcrypto-dilithium and pqcrypto-traits
use pqcrypto_dilithium::dilithium3;
use pqcrypto_traits::sign::{PublicKey, SignedMessage};

fn main() {
    // === Step 1: Generate a Key Pair ===
    // The key pair consists of:
    // - Public Key (pk): Used for signature verification
    // - Secret Key (sk): Used for signing messages
    let (pk, sk) = dilithium3::keypair();
    println!("Public Key: {:?}", pk.as_bytes());

    // === Step 2: Define the Message ===
    // This is the message that will be digitally signed.
    let message = b"Quantum Resistant Blockchain Message";

    // === Step 3: Sign the Message ===
    // The private key (sk) is used to create a digital signature for the message.
    // The output is a SignedMessage struct that includes both the message and its signature.
    let signed_message = dilithium3::sign(message, &sk);
    println!("Signed Message Length: {} bytes", signed_message.as_bytes().len());

    // === Step 4: Verify the Signed Message ===
    // Verification checks if the signature is valid using the corresponding public key.
    // If verification succeeds, it returns the original message.
    match dilithium3::open(&signed_message, &pk) {
        Ok(valid_message) => {
            println!("Verified successfully: {:?}", String::from_utf8_lossy(&valid_message));
        },
        Err(_) => {
            println!("Signature verification failed!");
        }
    }
}
