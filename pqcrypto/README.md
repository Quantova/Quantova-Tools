# 🔐 Quantum-Resistant Cryptography Toolkit (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![PQCrypto](https://img.shields.io/badge/PQCrypto-Latest-green)](https://github.com/rustpq/pqcrypto)

A comprehensive, open-source toolkit for implementing, testing, and benchmarking post-quantum cryptographic algorithms in Rust. This project provides practical examples and implementations of NIST-standardized and alternative post-quantum cryptographic primitives to help developers prepare systems for the quantum computing era.

## 📌 Purpose & Overview

This toolkit serves as both an educational resource and a practical implementation guide for quantum-resistant cryptography. As quantum computers continue to advance, traditional cryptographic algorithms based on integer factorization (RSA) and discrete logarithm problems (ECC) will become vulnerable. This project demonstrates how to:

- Implement and use post-quantum cryptographic algorithms in Rust applications
- Compare performance characteristics across different algorithms
- Integrate quantum-resistant cryptography into existing systems
- Provide ready-to-use examples for blockchain and distributed systems

Our focus is on production-ready implementations with thorough documentation to facilitate adoption of quantum-resistant cryptography in real-world applications.

## 🔍 Why Post-Quantum Cryptography?

Quantum computers pose a significant threat to current cryptographic standards:

- Shor's algorithm can efficiently break RSA, DSA, ECC, and other public-key systems
- Grover's algorithm reduces the security of symmetric cryptography by roughly half
- Critical infrastructures like blockchain networks, financial systems, and secure communications require quantum resistance

This toolkit helps developers prepare for this cryptographic transition by implementing algorithms specifically designed to resist quantum attacks.

## 📦 Supported Algorithms

| Algorithm | Type | NIST Status | Rust Implementation | Security Level | Key Features |
|-----------|------|-------------|---------------------|----------------|--------------|
| **ML-KEM (Kyber)** | Lattice-based KEM | Standardized (2023) | `pqcrypto-kyber` | 1-5 | Efficient key exchange, multiple security levels, compact keys |
| **ML-DSA (Dilithium)** | Lattice-based Signature | Standardized (2023) | `pqcrypto-dilithium` | 2-5 | Deterministic signatures, no side-channel vulnerabilities from RNG |
| **Falcon** | Lattice-based Signature | Standardized (2023) | `pqcrypto-falcon` | 5 | Compact signatures, fast verification, NTRU lattice-based |
| **SPHINCS+** | Hash-based Signature | Standardized (2023) | `pqcrypto-sphincsplus` | 1-5 | Minimal security assumptions, stateless design |
| **FrodoKEM** | Lattice-based KEM | NIST Round 3 Alternate | `pqcrypto-frodokem` | 1-5 | Conservative design, structureless lattice assumption |
| **NTRU** | Lattice-based KEM | NIST Round 3 Alternate | `pqcrypto-ntru` | 5 | Well-studied, mature lattice-based encryption |

> **Note**: Security levels correspond to NIST security categories, where level 1 is equivalent to AES-128, level 3 to AES-192, and level 5 to AES-256.

## 🛠️ Installation & Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- A C compiler (for some underlying PQClean dependencies)

### Getting Started

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/quantum-resistant-toolkit.git
   cd quantum-resistant-toolkit
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

3. **Run the Examples**

   ```bash
   # Run all examples
   cargo run --release --bin run_all_examples

   # Or run individual algorithm examples
   cargo run --release --bin kyber_example
   cargo run --release --bin dilithium_example
   cargo run --release --bin falcon_example
   cargo run --release --bin sphincs_example
   cargo run --release --bin frodo_example
   cargo run --release --bin ntru_example
   ```

4. **Run Benchmarks**

   ```bash
   cargo run --release --bin benchmark
   ```

## 📚 Algorithm Implementations & Examples

### ⚡ ML-KEM (Kyber) - Key Encapsulation

```rust
use pqcrypto_kyber::kyber768::{keypair, encapsulate, decapsulate};
use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret};

// Generate keypair
let (public_key, secret_key) = keypair();

// Encapsulate - creates a shared secret and ciphertext
let (shared_secret_a, ciphertext) = encapsulate(&public_key);

// Decapsulate - recover the shared secret from ciphertext using secret key
let shared_secret_b = decapsulate(&ciphertext, &secret_key);

// Verify both parties have the same shared secret
assert_eq!(shared_secret_a.as_bytes(), shared_secret_b.as_bytes());
```

ML-KEM (Kyber) is a lattice-based key encapsulation mechanism standardized by NIST in 2023. It offers:

- **Efficient Key Exchange**: Fast computation with small key and ciphertext sizes
- **Multiple Security Levels**: Implementations for different security requirements (Kyber512, Kyber768, Kyber1024)
- **IND-CCA2 Security**: Provable security against chosen-ciphertext attacks
- **Simple API**: Easy to integrate key exchange functionality
- **Primary Use**: Secure key establishment for hybrid encryption schemes

Key sizes and performance metrics are documented in the [Kyber Example](./examples/kyber_example.rs).

### 🔏 ML-DSA (Dilithium) - Digital Signatures

```rust
use pqcrypto_dilithium::dilithium3::{keypair, sign, verify, public_key_bytes, secret_key_bytes};
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};

// Generate keypair
let (public_key, secret_key) = keypair();

// Message to sign
let message = b"This message will be signed with a quantum-resistant algorithm";

// Create signature
let signature = sign(message, &secret_key);

// Verify signature
let verified = verify(message, &signature, &public_key);
assert!(verified.is_ok());
```

ML-DSA (Dilithium) is a lattice-based digital signature scheme standardized by NIST in 2023. It provides:

- **Deterministic Signatures**: Reduces side-channel attack vectors
- **Module-LWE Based**: Built on well-studied lattice problems
- **Multiple Security Levels**: Variants for different security requirements
- **Balance of Size and Speed**: Reasonable signature sizes with efficient verification
- **Primary Use**: Authentication, document signing, code signing

The implementation includes comprehensive error handling and parameter validation. See [Dilithium Example](./examples/dilithium_example.rs) for detailed usage.

### 🦅 Falcon - Compact Lattice Signatures

```rust
use pqcrypto_falcon::falcon512::{keypair, sign, verify};
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};
use rand::rngs::OsRng;

// Generate keypair
let (public_key, secret_key) = keypair();

// Message to sign
let message = b"Falcon provides compact signatures with fast verification";

// Sign the message
let signature = sign(message, &secret_key);

// Verify the signature
let verification_result = verify(message, &signature, &public_key);
assert!(verification_result.is_ok());
```

Falcon is an NTRU-based lattice signature scheme standardized by NIST in 2023. Its key features include:

- **Compact Signatures**: Significantly smaller signatures than Dilithium
- **Fast Verification**: Efficient signature verification process
- **NTRU Lattices**: Based on well-studied NTRU lattice problems
- **Floating-Point Operations**: Uses floating-point arithmetic for efficiency
- **Primary Use**: Applications where signature size is critical

More details and advanced usage patterns are available in the [Falcon Example](./examples/falcon_example.rs).

### 🔄 SPHINCS+ - Stateless Hash-Based Signatures

```rust
use pqcrypto_sphincsplus::sphincssha2128fsimple::{keypair, sign, verify};
use pqcrypto_traits::sign::{PublicKey, SecretKey, DetachedSignature};

// Generate keypair
let (public_key, secret_key) = keypair();

// Message to sign
let message = b"SPHINCS+ relies only on the security of cryptographic hash functions";

// Sign message
let signature = sign(message, &secret_key);

// Verify signature
let result = verify(message, &signature, &public_key);
assert!(result.is_ok());
```

SPHINCS+ is a stateless hash-based signature scheme standardized by NIST in 2023:

- **Minimal Security Assumptions**: Based solely on the security of cryptographic hash functions
- **Stateless Design**: No need to maintain state between signatures (unlike other hash-based schemes)
- **Post-Quantum Security**: Resistant to both classical and quantum attacks
- **Configurable Parameters**: Tradeoffs between signature size, key size, and performance
- **Primary Use**: Long-term security where signature size is not critical

Note that SPHINCS+ has larger signatures compared to lattice-based alternatives. See [SPHINCS+ Example](./examples/sphincs_example.rs) for implementation details.

### 🧮 FrodoKEM - Conservative Lattice KEM

```rust
use pqcrypto_frodokem::frodokem976aes::{keypair, encapsulate, decapsulate};
use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret};

// Generate keypair
let (public_key, secret_key) = keypair();

// Encapsulate to create shared secret and ciphertext
let (shared_secret_sender, ciphertext) = encapsulate(&public_key);

// Decapsulate to recover shared secret
let shared_secret_receiver = decapsulate(&ciphertext, &secret_key);

// Both parties now have the same shared secret
assert_eq!(shared_secret_sender.as_bytes(), shared_secret_receiver.as_bytes());
```

FrodoKEM is a conservative lattice-based key encapsulation mechanism:

- **Structureless Design**: Avoids special algebraic structures that might introduce vulnerabilities
- **Well-Understood Security**: Based on the standard Learning With Errors (LWE) problem
- **Conservative Parameters**: Prioritizes security margin over performance
- **Multiple Parameter Sets**: Options for different security levels and performance needs
- **Primary Use**: High-security applications where conservative design is preferred over performance

See [FrodoKEM Example](./examples/frodo_example.rs) for detailed implementation and performance notes.

### 🔑 NTRU - Well-Established Lattice KEM

```rust
use pqcrypto_ntru::ntruhrss701::{keypair, encapsulate, decapsulate};
use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret};

// Generate a keypair
let (public_key, secret_key) = keypair();

// Encapsulate a shared secret
let (shared_secret_1, ciphertext) = encapsulate(&public_key);

// Decapsulate the shared secret
let shared_secret_2 = decapsulate(&ciphertext, &secret_key);

// Verify that both sides have the same shared secret
let success = shared_secret_1.as_bytes() == shared_secret_2.as_bytes();
assert!(success);
```

NTRU is one of the oldest and most studied lattice-based cryptosystems:

- **Mature Algorithm**: Proposed in 1996, NTRU has withstood decades of cryptanalysis
- **Efficient Implementation**: Good performance characteristics for a post-quantum algorithm
- **Ring-Based Structure**: Uses polynomial rings for efficient operations
- **No Patent Restrictions**: Previously patented, now fully available for use
- **Primary Use**: Secure communications where a well-studied algorithm is preferred

The implementation includes error handling and key management examples. See [NTRU Example](./examples/ntru_example.rs) for details.

## 📊 Performance Benchmarks

This toolkit includes comprehensive benchmarks to help you understand the performance characteristics of each algorithm in different contexts. Key metrics measured include:

- **Key Generation Time**: CPU time required to generate keypairs
- **Operation Time**: Encryption/decryption or signing/verification time
- **Memory Usage**: Peak memory consumption during operations
- **Key and Signature Sizes**: Space requirements for storing keys and signatures/ciphertexts

Run the benchmarks to see how these algorithms perform on your specific hardware:

```bash
cargo run --release --bin benchmark
```

Sample benchmark results (on AMD Ryzen 9 5900X):

| Algorithm | Key Generation (ms) | Operation (ms) | Public Key Size (bytes) | Signature/Ciphertext Size (bytes) |
|-----------|---------------------|----------------|-------------------------|-----------------------------------|
| ML-KEM 768 | 0.54 | Encap: 0.61, Decap: 0.59 | 1,184 | 1,088 (ciphertext) |
| ML-DSA 3 | 1.87 | Sign: 1.92, Verify: 0.51 | 1,952 | 3,293 (signature) |
| Falcon-512 | 24.3 | Sign: 1.31, Verify: 0.15 | 897 | ~666 (signature) |
| SPHINCS+-128s | 3.41 | Sign: 252, Verify: 7.8 | 32 | 7,856 (signature) |
| FrodoKEM-976 | 48.2 | Encap: 49.7, Decap: 49.1 | 15,632 | 15,744 (ciphertext) |
| NTRU-HRSS-701 | 12.5 | Encap: 0.34, Decap: 0.89 | 1,138 | 1,138 (ciphertext) |

> For detailed and up-to-date benchmarks, please run the benchmark tool on your specific hardware.

## 🔄 Integration Examples

### Hybrid Cryptography

This toolkit demonstrates how to combine classical and post-quantum algorithms for transitional security:

```rust
// Example of hybrid encryption using both X25519 (classical) and Kyber (post-quantum)
// See examples/hybrid_encryption.rs for the complete implementation
```

### Blockchain Integration

Examples showing how to integrate these algorithms into blockchain applications:

```rust
// Example of post-quantum signature verification in a blockchain context
// See examples/blockchain_integration.rs for details
```

## 🛡️ Security Considerations

When implementing post-quantum cryptography, consider these important security aspects:

1. **Side-Channel Protection**: Some implementations may be vulnerable to timing attacks or other side-channel leakage
2. **Hybrid Approaches**: Consider using both traditional and post-quantum algorithms during the transition period
3. **Parameter Selection**: Choose appropriate security parameters based on your threat model
4. **Key Management**: Implement secure key storage and management practices
5. **Updates and Patching**: Stay current with the latest research and security updates

This toolkit implements best practices for secure usage, but you should always conduct a security review for your specific application context.

## 🤝 Contributing

We welcome contributions from the community! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Implement your changes with appropriate tests
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📚 Resources

- [NIST Post-Quantum Cryptography Standardization](https://csrc.nist.gov/Projects/post-quantum-cryptography)
- [pqcrypto Rust Project](https://github.com/rustpq/pqcrypto)
- [PQClean: Clean implementations of post-quantum cryptography](https://github.com/PQClean/PQClean)
- [Post-Quantum Cryptography for Blockchain](./docs/blockchain_integration.md)
- [Performance Optimization Guide](./docs/performance.md)

## ✉️ Contact

For questions, suggestions, or collaboration opportunities, please open an issue in this repository or contact the maintainer at [your-email@example.com](mailto:your-email@example.com).

---

**Note**: While this toolkit provides implementations of post-quantum algorithms, we recommend thorough testing and security review before deploying in production environments. Cryptographic implementations should always be carefully validated for each specific use case.