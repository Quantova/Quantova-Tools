//! ================================================================
//! 🧪 Post-Quantum Cryptography: FrodoKEM-976-AES 
//!
//! 🔐 Algorithm: FrodoKEM-976-AES
//! 📚 Standardization: NIST PQC Round 3 Alternate Finalist
//! 🧮 Type: Lattice-based Key Encapsulation Mechanism (KEM)
//!
//! 🛡️ Security Level: Approx. AES-192 equivalent
//! 🚫 Structureless Design: Does not rely on ideal lattices
//!
//! ✅ This example demonstrates:
//!     1. Keypair generation
//!     2. Shared secret encapsulation
//!     3. Ciphertext generation
//!     4. Shared secret decapsulation
//!     5. Byte comparison to verify secure key agreement
//!
//! ⚠️ Important:
//!     - Use compatible crate versions (see Cargo.toml)
//!     - Do not attempt to access inner `.0` field of keys or secrets (private)
//!     - Use `.as_bytes()` from `pqcrypto-traits`
//!
//! 🧱 Ideal for: Blockchain encryption layers, secure communication protocols,
//!              quantum-resistant wallets, identity systems, and more.
//!
//! 📦 Crates required:
//!     - pqcrypto-frodo
//!     - pqcrypto-traits
//!     - hex
//! ================================================================

use pqcrypto_frodo::frodokem976aes;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};
use hex;

fn main() {
    println!("🔐 Starting FrodoKEM-976-AES Key Exchange Example...\n");

    // ================================================================
    // STEP 1: Receiver (e.g., server or blockchain node) generates keypair
    // The public key will be shared with the sender
    // The secret key is kept private by the receiver
    // ================================================================
    let (pk, sk) = frodokem976aes::keypair();

    println!("📤 Public Key (Receiver):");
    println!("{}", hex::encode(pk.as_bytes()));
    println!("✅ Public Key generated ({} bytes)\n", pk.as_bytes().len());

    // ================================================================
    // STEP 2: Sender encapsulates a shared secret using receiver's public key
    // This creates:
    //   - A random shared secret
    //   - A ciphertext that can be sent publicly
    // ================================================================
    let (ss_sender, ct) = frodokem976aes::encapsulate(&pk);

    println!("📦 Ciphertext (to be sent to receiver):");
    println!("{}", hex::encode(ct.as_bytes()));
    println!("✅ Ciphertext generated ({} bytes)\n", ct.as_bytes().len());

    println!("🔑 Shared Secret (Sender side):");
    println!("{}", hex::encode(ss_sender.as_bytes()));
    println!("✅ Shared Secret generated ({} bytes)\n", ss_sender.as_bytes().len());

    // ================================================================
    // STEP 3: Receiver decapsulates the ciphertext using their secret key
    // This should derive the same shared secret as the sender
    // ================================================================
    let ss_receiver = frodokem976aes::decapsulate(&ct, &sk);

    println!("🔑 Shared Secret (Receiver side):");
    println!("{}", hex::encode(ss_receiver.as_bytes()));
    println!("✅ Shared Secret recovered ({} bytes)\n", ss_receiver.as_bytes().len());

    // ================================================================
    // STEP 4: Verify if shared secrets match
    // This is the basis of secure key exchange!
    // ================================================================
    if ss_sender.as_bytes() == ss_receiver.as_bytes() {
        println!("🎉 ✅ Shared secrets match!");
        println!("🔐 FrodoKEM-976-AES key exchange was successful and secure.\n");
    } else {
        println!("❌ Shared secrets DO NOT match!");
        println!("⚠️ Key exchange failed. Do not use this key for secure communication.\n");
    }

    println!("🧪 FrodoKEM-976-AES example completed.");
}
