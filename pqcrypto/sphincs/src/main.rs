// Quantum-Resistant Digital Signature Example using SPHINCS+
//
// This code demonstrates the use of the post-quantum cryptographic algorithm SPHINCS+.
// SPHINCS+ is a stateless hash-based digital signature scheme designed for security against quantum attacks.
// The program follows these steps:
// 1. Generate a public-secret key pair
// 2. Sign a message using the private key
// 3. Verify the signature using the public key

// Import necessary cryptographic libraries from pqcrypto-sphincsplus and pqcrypto-traits
use pqcrypto_sphincsplus::sphincssha256128frobust;
use pqcrypto_traits::sign::{PublicKey, SignedMessage};
use std::str;

fn main() {
    // === Step 1: Generate a Key Pair ===
    let (pk, sk) = sphincssha256128frobust::keypair();
    println!("Public Key: {:?}", pk.as_bytes());

    // === Step 2: Define the Message ===
    let message = b"Quantum Resistant Blockchain Message";

    // === Step 3: Sign the Message ===
    let signed_message = sphincssha256128frobust::sign(message, &sk);
    println!(
        "Signed Message Length: {} bytes",
        signed_message.as_bytes().len()
    );

    // === Step 4: Verify the Signed Message ===
    match sphincssha256128frobust::open(&signed_message, &pk) {
        Ok(valid_message) => {
            // Convert Box<[u8]> to &str safely
            let msg_str = str::from_utf8(&valid_message).expect("Invalid UTF-8 data");
            println!("Verified successfully: {}", msg_str);
        }
        Err(_) => {
            println!("Signature verification failed!");
        }
    }
}
