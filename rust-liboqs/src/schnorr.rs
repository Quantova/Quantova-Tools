use oqs::sig::{Algorithm, Sig, Signature, PublicKey, SecretKey};
use rand::random;

struct PQSchnorr {
    public_key: PublicKey,
    secret_key: SecretKey,
}

impl PQSchnorr {
    fn new() -> Self {
        let sig = Sig::new(Algorithm::Dilithium3).unwrap();
        let (public_key, secret_key) = sig.keypair().unwrap();
        println!(" Post-Quantum Schnorr Key Pair Generated!");
        println!("Public Key: {:?}", public_key);
        println!("Secret Key: {:?}", secret_key);
        Self { public_key, secret_key }
    }

    fn sign(&self, message: &[u8]) -> Signature {
        let sig = Sig::new(Algorithm::Dilithium3).unwrap();
        let signature = sig.sign(message, &self.secret_key).unwrap();
        println!("Signature created for message: {:?}", String::from_utf8_lossy(message));
        println!("Signature: {:?}", signature);
        signature
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        let sig = Sig::new(Algorithm::Dilithium3).unwrap();
        let result = sig.verify(message, signature, &self.public_key).is_ok();
        if result {
            println!("✅ Signature verification successful!");
        } else {
            println!("❌ Signature verification failed!");
        }
        result
    }
}

pub fn schnorr() {
    let message = b"Post-Quantum Schnorr Signature Example";
    println!("📝 Message: {}",
        String::from_utf8_lossy(message));

    let pq_schnorr = PQSchnorr::new();

    // Sign the message
    let signature = pq_schnorr.sign(message);

    // Verify the signature
    println!("🔍 Verifying Signature...");
    pq_schnorr.verify(message, &signature);
}
