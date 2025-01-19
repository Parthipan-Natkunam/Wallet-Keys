use anyhow::{anyhow, Result};
use bip39::{Language, Mnemonic};
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use clap::Parser;
use colored::*;
use hex;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};

const ETH_COIN_TYPE: u32 = 60;

#[derive(Parser, Debug)]
#[command(version, about = "BIP44 Compatible Wallet Keys Generator for Ethereum Network")]
struct Args {
    /// The mnemonic phrase (12 or 24 words)
    #[arg(long)]
    mnemonic: String,

    /// Number of addresses to generate (default: 1)
    #[arg(long, default_value = "1")]
    num_addresses: u32,

    /// The account index (default: 0)
    #[arg(long, default_value = "0")]
    account: u32,
}

fn generate_eth_address(public_key: &[u8]) -> String {
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];

    // Remove the first byte (0x04) which indicates uncompressed public key
    keccak.update(&public_key[1..]);
    keccak.finalize(&mut hash);

    // Take last 20 bytes of the hash to get ethereum address
    let address = &hash[12..];
    format!("0x{}", hex::encode(address))
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate and create mnemonic
    let mnemonic = Mnemonic::parse_in(Language::English, &args.mnemonic)
        .map_err(|e| anyhow!("Invalid mnemonic: {}", e))?;

    // Generate seed from mnemonic
    let seed = mnemonic.to_seed("");

    // Create master key
    let secp = Secp256k1::new();
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
        .map_err(|e| anyhow!("Failed to create master key: {}", e))?;

    println!("\n{}", "Master Key Information:".green().bold());
    println!("Master Private Key: {}", master_key);

    let master_public_key = ExtendedPubKey::from_priv(&secp, &master_key);
    println!("Master Public Key: {}", master_public_key);

    // BIP44 path structure: m/44'/60'/account'/0/index
    println!("\n{}", "Generated Ethereum Addresses:".green().bold());

    for i in 0..args.num_addresses {
        // Construct BIP44 path for Ethereum
        let path = format!("m/44'/{}'/{:?}'/0/{}", ETH_COIN_TYPE, args.account, i);
        let derivation_path = DerivationPath::from_str(&path)
            .map_err(|e| anyhow!("Invalid derivation path: {}", e))?;

        // Derive child key
        let child_priv = master_key
            .derive_priv(&secp, &derivation_path)
            .map_err(|e| anyhow!("Failed to derive child key: {}", e))?;

        let child_pub = ExtendedPubKey::from_priv(&secp, &child_priv);
        let public_key = child_pub.public_key.serialize_uncompressed();

        println!("\n{}", format!("Address {}", i).yellow().bold());
        println!("Path: {}", path);
        println!(
            "Private Key: {}",
            hex::encode(&child_priv.private_key.secret_bytes())
        );
        println!("Public Key: 0x{}", hex::encode(&public_key));

        // Generate Ethereum address
        let eth_address = generate_eth_address(&public_key);
        println!("Ethereum Address: {}", eth_address);
    }

    Ok(())
}
