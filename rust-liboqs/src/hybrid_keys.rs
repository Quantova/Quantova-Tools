
use oqs::sig::{self, Algorithm, Sig};
use ring::signature::{Ed25519KeyPair, KeyPair, Signature, ED25519};
use ring::rand::{SystemRandom, SecureRandom};
use hex;

fn sign_classically(data: &[u8], private_key: &Ed25519KeyPair) -> Signature {
    private_key.sign(data)
}

fn verify_classically(data: &[u8], signature: &Signature, public_key: &[u8]) -> bool {
    ring::signature::UnparsedPublicKey::new(&ED25519, public_key)
        .verify(data, signature.as_ref())
        .is_ok()
}

pub fn hybrid_keys() {
    // Simulated data to sign
    let data = b"hybrid cryptography message!";
    println!("\n=============================");
    println!(" Hybrid Cryptography Demo.");
    println!("=============================");
    println!(" Data to be signed: {}", String::from_utf8_lossy(data));

    // Generate a classical Ed25519 key pair
    let rng = SystemRandom::new();
    let private_key_bytes = {
        let mut key = [0u8; 32];
        rng.fill(&mut key).unwrap();
        key
    };
    let private_key = Ed25519KeyPair::from_seed_unchecked(&private_key_bytes).unwrap();
    let classic_signature = sign_classically(data, &private_key);
    let classic_public_key = private_key.public_key().as_ref().to_vec();
    
    println!("\n Classical Ed25519 Key Pair:");
    println!("   - Public Key: {}", hex::encode(&classic_public_key));
    println!("   - Signature : {}", hex::encode(classic_signature.as_ref()));

    // Generate a PQC signature (Dilithium2)
    let sig = Sig::new(Algorithm::Dilithium2).unwrap();
    let (pqc_public_key, pqc_private_key) = sig.keypair().unwrap();
    let pqc_signature = sig.sign(data, &pqc_private_key).unwrap();

    println!("\n PQC Dilithium2 Key Pair:");
    println!("   - Public Key: {}", hex::encode(&pqc_public_key));
    println!("   - Signature : {}", hex::encode(&pqc_signature));

    // Hybrid signature (simple concatenation for demonstration)
    let hybrid_signature = [classic_signature.as_ref(), pqc_signature.as_ref()].concat();
    println!("\n🔗 Hybrid Signature:");
    println!("   - Signature: {}", hex::encode(&hybrid_signature));

    // Verification
    let classic_valid = verify_classically(data, &classic_signature, &classic_public_key);
    let pqc_valid = sig.verify(data, &pqc_signature, &pqc_public_key).is_ok();

    println!("\n=============================");
    println!(" Verification Results:");
    println!("=============================");
    println!(" Classic Ed25519 Signature Valid: {}", if classic_valid { "✅ Valid" } else { "❌ Invalid" });
    println!(" PQC Dilithium2 Signature Valid: {}", if pqc_valid { "✅ Valid" } else { "❌ Invalid" });

    if classic_valid && pqc_valid {
        println!("\n All signatures are valid! Hybrid signature is secure!");
    } else {
        println!("\n❌ Signature verification failed!");
    }
}
