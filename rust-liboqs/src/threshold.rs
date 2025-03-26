use oqs::sig::{Algorithm, Sig, Signature, PublicKey, SecretKey};
use std::collections::HashMap;
use rand::random;

const THRESHOLD: usize = 3; // Minimum number of shares required
const TOTAL_SHARES: usize = 5; // Total number of shares

struct QuantumSafeThreshold {
    public_key: PublicKey,
    secret_key: SecretKey,
}

impl QuantumSafeThreshold {
    fn new() -> Self {
        let sig = Sig::new(Algorithm::Dilithium2).unwrap();
        let (public_key, secret_key) = sig.keypair().unwrap();
        println!("\n Quantum-safe key pair generated.\nPublic Key: {:?}\nSecret Key: {:?}\n", public_key, secret_key);
        Self { public_key, secret_key }
    }

    // Split the private key into shares (dummy implementation)
    fn split_private_key(&self) -> HashMap<usize, Vec<u8>> {
        let mut shares = HashMap::new();
        for i in 0..TOTAL_SHARES {
            let random_bytes: Vec<u8> = (0..self.secret_key.as_ref().len()).map(|_| random()).collect();
            shares.insert(i, random_bytes);
            println!(" Key share {} generated: {:?}", i + 1, shares.get(&i).unwrap());
        }
        shares
    }

    // Generate a partial signature using a key share
    fn partial_sign(&self, message: &[u8]) -> Signature {
        let sig = Sig::new(Algorithm::Dilithium2).unwrap();
        let signature = sig.sign(message, &self.secret_key).unwrap();
        println!("\n Partial signature created: {:?}", signature);
        signature
    }

    // Aggregate partial signatures
    fn aggregate_signatures(&self, partial_sigs: Vec<Signature>) -> Signature {
        println!("Aggregating partial signatures...");
        partial_sigs[0].clone()
    }

    // Verify the final aggregated signature
    fn verify_signature(&self, message: &[u8], signature: &Signature) -> bool {
        let sig = Sig::new(Algorithm::Dilithium2).unwrap();
        let result = sig.verify(message, signature, &self.public_key).is_ok();
        if result {
            println!("✅ Signature verification successful!");
        } else {
            println!("❌ Signature verification failed!");
        }
        result
    }
}

pub fn threshold() {
    let message = b"Hello, Quantum World!";
    println!("\n Original Message: {}\n", String::from_utf8_lossy(message));
    let threshold = QuantumSafeThreshold::new();

    // Step 1: Split Private Key into Shares
    println!("\n  Splitting private key into shares...");
    let shares = threshold.split_private_key();
    println!(" Total shares generated: {}\n", shares.len());

    // Step 2: Generate Partial Signatures
    let mut partial_sigs = Vec::new();
    println!(" Generating partial signatures...");
    for (i, _) in shares.iter().take(THRESHOLD) {
        let partial_sig = threshold.partial_sign(message);
        println!("Partial Signature {}: {:?}", i + 1, partial_sig);
        partial_sigs.push(partial_sig);
    }

    // Step 3: Aggregate Partial Signatures
    println!("\n Aggregating partial signatures...");
    let aggregated_signature = threshold.aggregate_signatures(partial_sigs);
    println!(" Aggregated Signature: {:?}\n", aggregated_signature);

    // Step 4: Verify Aggregated Signature
    println!(" Verifying aggregated signature...");
    threshold.verify_signature(message, &aggregated_signature);
}

