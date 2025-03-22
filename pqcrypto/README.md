# 🔐 Quantum-Resistant Cryptography Toolkit (Rust)

This project is a Post-Quantum Cryptography (PQC) testing toolkit built in Rust, showcasing implementation and validation of various NIST-selected and recommended algorithms. These include lattice-based, hash-based, and code-based cryptographic primitives designed to remain secure even in a post-quantum world.

## 📌 Purpose

To test and understand the functionality, performance, and implementation of **quantum-resistant cryptographic algorithms**, particularly in the context of integrating them into blockchain and Web3 applications.

---

## 📦 Algorithms Tested

| Algorithm     | Type            | NIST Status       | Crate Used           | Use Case             |
|---------------|------------------|-------------------|-----------------------|----------------------|
| **Kyber**     | Lattice-based KEM | Standardized      | `pqcrypto_kyber`      | Key exchange         |
| **Dilithium** | Lattice-based Sig | Standardized      | `pqcrypto_dilithium`  | Digital signatures   |
| **Falcon**    | Lattice-based Sig | Standardized      | `pqcrypto_falcon`     | Digital signatures   |
| **SPHINCS+**  | Hash-based Sig    | Standardized      | `pqcrypto_sphincsplus`| Stateless signatures |
| **FrodoKEM**  | Lattice-based KEM | Round 3 Alternate | `pqcrypto`            | Key exchange         |

---

## 🛠️ Setup Instructions

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/quantum_resistant_toolkit.git
   cd quantum_resistant_toolkit


## 🧪 Algorithm Demos

### ✅ Kyber (CRYSTALS-Kyber)
- **Type**: Key Encapsulation Mechanism (KEM)
- **Demonstrates**:
  - Key pair generation
  - Encapsulation
  - Decapsulation
  - Shared secret verification

### ✅ Dilithium (CRYSTALS-Dilithium)
- **Type**: Digital Signature
- **Demonstrates**:
  - Key pair generation
  - Message signing
  - Signature verification

### ✅ Falcon
- **Type**: Digital Signature
- **Demonstrates**:
  - Compact lattice-based signatures
  - High performance verification

### ✅ SPHINCS+
- **Type**: Stateless Hash-Based Signature
- **Demonstrates**:
  - Quantum-secure signature with no state management
  - Hash-based security assumptions

### ✅ FrodoKEM
- **Type**: Lattice-based KEM (structureless)
- **Demonstrates**:
  - Conservative design
  - Key encapsulation and secure decapsulation
