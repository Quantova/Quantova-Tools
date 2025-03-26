# Post-Quantum Cryptography and Blockchain Projects

This document provides an overview of various projects and libraries related to post-quantum cryptography, including cryptographic libraries, blockchain integrations, and enterprise solutions.

---

## 1. Open Quantum Safe (liboqs)
**Description:**  
A C library that aggregates numerous post-quantum key encapsulation mechanisms (KEMs) and digital signature schemes via a common API. It also provides a test harness and benchmarking routines.

- **GitHub:** [open-quantum-safe/liboqs](https://github.com/open-quantum-safe/liboqs)
- **Features:** Supports multiple algorithms (BIKE, Classic McEliece, FrodoKEM, Kyber, ML-KEM, Falcon, SPHINCS+, XMSS, etc.), integrates with OpenSSL, provides language wrappers, and supports benchmarking.

### Next Steps:
- Decide whether to include C libraries or focus on Rust-based options.
- Evaluate supported algorithms for performance and security compliance.
- Explore language wrappers (Python, Rust) for integration.

---

## 2. QuantCrypt (Python)
**Description:**  
A cross-platform Python library leveraging precompiled PQClean binaries for post-quantum cryptography.

- **GitHub:** [aabmets/quantcrypt](https://github.com/aabmets/quantcrypt)
- **Key Features:** Easy-to-use interfaces for KEMs, digital signature schemes, and symmetric primitives.

### Next Steps:
- Investigate integration with Python-based DApps and back-end services.

---

## 3. rustpq/pqcrypto (Rust)
**Description:**  
Rust bindings for post-quantum cryptographic algorithms originally collected by the PQClean project.

- **GitHub:** [rustpq/pqcrypto](https://github.com/rustpq/pqcrypto)
- **Features:** Provides idiomatic Rust interfaces for lattice-based, hash-based, and other cryptographic schemes.

### Next Steps:
- Explore available bindings and test interoperability with Rust-based services.

---

## 4. noble-post-quantum (JavaScript/TypeScript)
**Description:**  
A minimal JavaScript/TypeScript library implementing post-quantum algorithms for web and Node.js environments.

- **GitHub:** [paulmillr/noble-post-quantum](https://github.com/paulmillr/noble-post-quantum)
- **Key Features:** Implements ML-KEM (CRYSTALS-Kyber), ML-DSA (CRYSTALS-Dilithium), and SLH-DSA (SPHINCS+).

### Next Steps:
- Integrate noble-post-quantum into front-end DApps.
- Verify performance across various runtime environments (browser, Node.js, React Native).

---

## 5. mupq/pqm4 (Embedded Systems)
**Description:**  
A library for post-quantum key encapsulation and digital signature algorithms optimized for ARM Cortex-M4 microcontrollers.

- **GitHub:** [mupq/pqm4](https://github.com/mupq/pqm4)
- **Features:** Provides speed, memory usage, and code size benchmarks for embedded systems.

### Next Steps:
- Test key PQ algorithms on target embedded hardware.

---

## 6. Quantum Resistant Ledger (QRL)
**Description:**  
A blockchain platform that uses XMSS (hash-based signatures) for quantum resistance.

- **GitHub:** [theQRL/QRL](https://github.com/theQRL/QRL)
- **Key Features:** Uses hash-based one-time signature schemes (XMSS) for security.

### Next Steps:
- Deploy QRL nodes for performance and security evaluation.
- Analyze transition strategies for conventional ledgers.

---

## 7. BTQ – Quantum-Resistant Bitcoin Ledger
**Description:**  
A Bitcoin fork integrating quantum-resistant techniques using XMSS.

- **GitHub:** [BitcoinQ/BTQ](https://github.com/BitcoinQ/BTQ)
- **Key Features:** Adds forward secrecy using one-time signatures.

### Next Steps:
- Investigate BTQ’s integration with Bitcoin protocols.
- Explore hybrid solutions for classical and quantum-resistant transactions.

---

## 8. QuantumBlockchainV1
**Description:**  
A blockchain implementation integrating quantum computing principles for enhanced security.

- **GitHub:** [OneNessQBAI/QuantumBlockchainV1](https://github.com/OneNessQBAI/QuantumBlockchainV1)
- **Features:** Merges blockchain and quantum cryptographic techniques.

### Next Steps:
- Evaluate smart contract deployment mechanisms.
- Test interoperability with existing DApps and blockchain networks.

---

## 9. Quantum-Resistant Bitcoin Transaction Framework (QR-BTF)
**Description:**  
A framework integrating post-quantum cryptographic algorithms into Bitcoin-like blockchains.

- **GitHub:** [detasecure/bitcolympics](https://github.com/detasecure/bitcolympics)
- **Features:** Cross-chain compatibility, Layer 2 integration.

### Next Steps:
- Investigate integration with existing blockchain infrastructure.
- Pilot QR-BTF as a Layer 2 security solution.

---

## 10. Quantum-Mesh
**Description:**  
A repository for developing quantum-resistant cryptographic protocols in blockchain platforms.

- **GitHub:** [KOSASIH/Quantum-Mesh](https://github.com/KOSASIH/Quantum-Mesh)
- **Key Features:** Provides a toolkit for protocol development and blockchain security enhancements.

### Next Steps:
- Identify key Quantum-Mesh components for blockchain integration.

---

## 11. Aggregated GitHub Topic: Post-Quantum Cryptography
**Description:**  
A curated GitHub topic aggregating post-quantum cryptography projects.

- **Link:** [GitHub Topic: post-quantum-cryptography](https://github.com/topics/post-quantum-cryptography)
- **Key Features:** Tracks emerging libraries, benchmarks, and best practices.

### Next Steps:
- Assess which libraries are relevant for the toolkit, prioritizing Rust-based solutions.
- Consider additional languages like Python.

---

## 12. Blockchain Integration Projects (Quantum-Blockchains)
**Description:**  
A collection of repositories integrating quantum-resistant techniques into blockchain projects.

- **GitHub:** [Quantum-Blockchains](https://github.com/Quantum-Blockchains)
- **Features:** Various implementations in Rust, JavaScript, and other languages.

### Next Steps:
- Evaluate integration feasibility.
- Consider using PqClean (C version of pqcrypto Rust).

---

## 13. Hyperledger Ursa (Enterprise Blockchain Integration)
**Description:**  
A modular cryptographic library by the Hyperledger Foundation for secure cryptographic primitives.

- **GitHub:** [hyperledger/ursa](https://github.com/hyperledger/ursa)
- **Key Features:** Supports modular cryptographic primitives across Hyperledger projects.

### Next Steps:
- Evaluate Ursa’s support for PQ algorithms.
- Pilot quantum-resistant modules in Hyperledger blockchains.
- Engage with the Hyperledger community for collaborative development.

---

## Conclusion
This document outlines various post-quantum cryptography and blockchain projects. Depending on the requirements, Rust-based solutions such as `pqcrypto` and `PqClean` may be prioritized, while additional projects like QuantCrypt (Python) and noble-post-quantum (JavaScript) may be considered for broader ecosystem integration.
