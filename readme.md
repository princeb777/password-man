# Password Manager CLI

A simple, command-line password manager written in Rust.

## Features

*   **Secure Storage:** Passwords are encrypted using AES-256-GCM with a key derived from a master password using Argon2.
*   **Password Generation:** Generate strong, random passwords of a specified length.
*   **List Passwords:** View all your saved passwords.
*   **Simple CLI:** Easy-to-use command-line interface.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/your-username/password-man.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd password-man
    ```
3.  Build the project:
    ```bash
    cargo build --release
    ```

### Usage

1.  Run the application:
    ```bash
    cargo run
    ```
2.  Enter a master password. This will be used to encrypt and decrypt your passwords.
3.  Choose an option from the menu:
    *   `[1]` Generate a new password.
    *   `[2]` Show all saved passwords.
    *   `[3]` Exit.

## Security

The security of your passwords is a top priority. This project uses the following cryptographic libraries and methods:

*   **Key Derivation:** [Argon2](https://en.wikipedia.org/wiki/Argon2) is used to derive a strong encryption key from your master password. This makes it difficult for attackers to guess your master password, even if they have access to the encrypted password file.
*   **Encryption:** [AES-256-GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode) is used to encrypt your passwords. This is a modern, authenticated encryption cipher that provides both confidentiality and integrity.

## Dependencies

*   [rand](https://crates.io/crates/rand)
*   [aes-gcm](https://crates.io/crates/aes-gcm)
*   [aes](https://crates.io/crates/aes)
*   [argon2](https://crates.io/crates/argon2)
*   [colored](https://crates.io/crates/colored)
