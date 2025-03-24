use oqs::sig::{self, Sig, Signature};
use std::fs::File;
use std::io::{self, Write};
use std::process;

struct QuantumSafeAuth {
    public_key: sig::PublicKey,
    secret_key: sig::SecretKey,
}

impl QuantumSafeAuth {
    fn new() -> Self {
        let sig = Sig::new(oqs::sig::Algorithm::Dilithium2).expect("Failed to create signature scheme.");
        let (public_key, secret_key) = sig.keypair().expect("Key pair generation failed.");
        println!(" Quantum-safe key pair generated.
        Public Key: {:?}
         Secret Key: {:?}", public_key, secret_key);
        Self {
            public_key,
            secret_key,
        }
    }

    fn sign_message(&self, message: &[u8]) -> Signature {
        let sig = Sig::new(oqs::sig::Algorithm::Dilithium2).expect("Failed to create signature scheme.");
        sig.sign(message, &self.secret_key).expect("Signing failed.")
    }

    fn verify_signature(&self, message: &[u8], signature: &Signature) -> bool {
        let sig = Sig::new(oqs::sig::Algorithm::Dilithium2).expect("Failed to create signature scheme.");
        sig.verify(message, signature, &self.public_key).is_ok()
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(self.public_key.as_ref())?;
        Ok(())
    }

    fn terminate(&self) {
        println!("Terminating the process.");
        process::exit(0);
    }
}

pub fn authentication() {
    let auth = QuantumSafeAuth::new();

    let message = b"Quantum-safe authentication message";
    println!("Message: {:?}", String::from_utf8_lossy(message));

    let signature = auth.sign_message(message);
    println!("Signature generated.");

    if auth.verify_signature(message, &signature) {
        println!("Signature verification successful!");
    } else {
        println!("Signature verification failed!");
    }

    match auth.save_to_file("public_key.bin") {
        Ok(_) => println!("Public key saved to file."),
        Err(e) => eprintln!("Failed to save public key: {}", e),
    }

    auth.terminate();
}
