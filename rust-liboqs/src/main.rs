mod authentication;
mod hybrid_keys;
mod schnorr;
mod threshold;

use std::io::{self, Write};

fn main() {
    loop {
        println!("\n==============================");
        println!(" Quantum Cryptography Toolkit");
        println!("==============================");
        println!("1. Quantum-Safe Authentication");
        println!("2. Hybrid Cryptography");
        println!("3. Post-Quantum Schnorr Signatures");
        println!("4. Threshold Signatures");
        println!("5. Exit");
        print!("\nSelect an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");

        match choice.trim() {
            "1" => {
                println!("\n Running Quantum-Safe Authentication...");
                authentication::authentication();
            }
            "2" => {
                println!("\n Running Hybrid Cryptography...");
                hybrid_keys::hybrid_keys();
            }
            "3" => {
                println!("\n Running Post-Quantum Schnorr Signatures...");
                schnorr::schnorr();
            }
            "4" => {
                println!("\n Running Threshold Signatures...");
                threshold::threshold();
            }
            "5" => {
                println!("🚪 Exiting...");
                break;
            }
            _ => println!("❌ Invalid option. Please try again."),
        }
    }
}
