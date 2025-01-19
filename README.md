# BIP44 Ethereum Wallet CLI

A command-line tool to generate Ethereum wallet addresses and keys using the BIP44 standard from a mnemonic phrase.

>[!WARNING]
>This tool is for educational and development purposes only. The Author is not responsible for any loss of funds or any other damages. Never share your mnemonic phrase or private keys.

## Features

- Generate Ethereum addresses from BIP44 derivation paths
- Support for multiple address generation
- Display private keys, public keys, and addresses
- BIP39 mnemonic support
- Colored output for better readability

## Installation

### From Source

1. Clone the repository:

```bash
git clone https://github.com/Parthipan-Natkunam/Wallet-Keys
cd Wallet-Keys
```

2. Build and install:

```bash
cargo install --path .
```

## Usage

Basic usage:

```bash
wallet-keygen --mnemonic "your twelve or twenty four word mnemonic phrase"
```

Options:

- `--mnemonic <PHRASE>`: Your BIP39 mnemonic phrase (12 or 24 words)
- `--num-addresses <NUMBER>`: Number of addresses to generate (default: 1)
- `--account <NUMBER>`: Account index for derivation path (default: 0)

Example:

```bash
wallet-keygen --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --num-addresses 3
```

This will generate:

- Master key information
- Multiple Ethereum addresses with their:
  - Derivation path
  - Private key
  - Public key
  - Ethereum address

## Security Notes

- Never share your mnemonic phrase or private keys
- This tool is for educational and development purposes
- Always verify generated addresses before use
- Consider using hardware wallets for significant amounts

## License

This project is licensed under the MIT License - see the LICENSE file for details.
