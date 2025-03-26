# Quantum Cryptography Toolkit

This toolkit demonstrates the integration of quantum-safe cryptographic algorithms into blockchain ecosystems. It leverages **liboqs-rust** for post-quantum cryptography and **ring** for classical cryptographic algorithms. The toolkit supports various cryptographic functionalities including hybrid cryptography, post-quantum Schnorr signatures, and threshold signatures.

## Features

1. **Quantum-Safe Authentication:**
   - Uses Dilithium2 for digital signatures.
   - Generates and verifies quantum-safe signatures.
   - Saves public keys to a file for verification.

2. **Hybrid Cryptography:**
   - Combines classical Ed25519 signatures with quantum-safe Dilithium2 signatures.
   - Verifies both classical and post-quantum signatures independently.

3. **Post-Quantum Schnorr Signatures:**
   - Implements a post-quantum variant of Schnorr signatures using Dilithium3.
   - Signs and verifies messages securely.

4. **Threshold Signatures:**
   - Demonstrates a basic implementation of threshold signature schemes.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- `liboqs` and `ring` libraries

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/quantum-crypto-toolkit.git
   cd quantum-crypto-toolkit
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the toolkit:
   ```bash
   cargo run
   ```

## Usage

Select a cryptographic functionality from the menu and follow the on-screen instructions. Each functionality demonstrates key generation, signing, and verification.

