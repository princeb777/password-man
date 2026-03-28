# 🔐 Password Manager CLI (Rust)

A minimal, secure, and lightweight **command-line password manager** built in Rust. This project focuses on strong cryptography, simplicity, and zero external services — your data stays entirely local.

---

## ✨ Features

* 🔒 **Secure Encryption**
  Uses **AES-256-GCM** for authenticated encryption.

* 🧠 **Strong Key Derivation**
  Master password is processed using **Argon2id** to resist brute-force attacks.

* 🎲 **Password Generator**
  Generates strong passwords with a mix of:

  * Lowercase
  * Uppercase
  * Numbers
  * Symbols

* 📋 **Password Vault**
  Store, retrieve, and manage passwords locally.

* 🎨 **Readable CLI UI**
  Clean terminal output with colors using the `colored` crate.

---

## 🏗️ Project Structure

```
password-man/
│
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── password_generator.rs  # Password generation logic
│   └── vault.rs               # Encryption + storage
│
├── Cargo.toml
└── README.md
```

---

## ⚙️ Installation

### 1. Clone the repository

```bash
git clone https://github.com/princeb777/password-man.git
cd password-man
```

### 2. Build the project

```bash
cargo build --release
```

---

## 🚀 Usage

Run the CLI:

```bash
cargo run
```

### Workflow

1. Enter your **master password**
2. Choose an option:

```
[1] Generate a new password
[2] Show all saved passwords
[3] Exit
```

### Example

```
🔑 Enter master password:
➜ ********

🔐 Password Manager CLI
[1] Generate a new password
[2] Show all saved passwords
[3] Exit
➜ Choose an option: 1

Generated password: A9#kL2!xQp
Do you want to save this password? (y/n)
```

---

## 🔐 Security Details

### Key Derivation

* Algorithm: **Argon2id**
* Purpose: Prevent brute-force attacks on the master password
* Output: 256-bit key

### Encryption

* Algorithm: **AES-256-GCM**
* Provides:

  * Confidentiality
  * Integrity (tamper detection)

### File Format

Encrypted vault file (`passwords.txt`) structure:

```
[salt (16 bytes)] + [nonce (12 bytes)] + [ciphertext]
```

### Important Notes

⚠️ If you forget your master password, your data **cannot be recovered**.

⚠️ This project is for **learning and personal use** — not audited for production use.

---

## 📦 Dependencies

* `rand` — Random number generation
* `aes-gcm` — Authenticated encryption
* `aes` — AES primitives
* `argon2` — Secure password hashing
* `colored` — Terminal styling

---

## 🧪 Limitations

* No search/filter functionality
* No password editing or deletion
* Stores data in a single local file
* No cross-device sync

---

## 🚧 Future Improvements

* [ ] Add password delete/edit
* [ ] Add search functionality
* [ ] Clipboard copy support
* [ ] Configurable password length & rules
* [ ] Better CLI (flags instead of menu)
* [ ] File locking / concurrency safety

---

## 🤝 Contributing

Contributions are welcome!

1. Fork the repo
2. Create a new branch
3. Submit a pull request

---

## 📜 License

MIT License

---

## 🙌 Acknowledgements

* Rust Crypto ecosystem
* Argon2 specification
* AES-GCM standard

---

## ⭐ If you like this project

Give it a star on GitHub — it helps a lot!
