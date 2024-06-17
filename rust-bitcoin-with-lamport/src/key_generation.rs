use rand::Rng;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use hex;

const KEY_SIZE: usize = 256;

fn generate_key_pair() -> (Vec<[u8; 32]>, Vec<[u8; 32]>) {
    let mut rng = rand::thread_rng();
    let mut private_key = Vec::new();
    let mut public_key = Vec::new();
    
    for _ in 0..KEY_SIZE {
        let mut sk = [0u8; 32];
        rng.fill(&mut sk);
        let pk = Sha256::digest(&sk);
        
        private_key.push(sk);
        public_key.push(pk.into());
    }
    
    (private_key, public_key)
}

pub fn generate_and_write_key_pairs(count: usize) -> std::io::Result<()> {
    let mut private_file = BufWriter::new(File::create("private_keys.txt")?);
    let mut public_file = BufWriter::new(File::create("public_keys.txt")?);

    for i in 0..count {
        let (private_key, public_key) = generate_key_pair();

        private_file.write_all(format!("Private Key {}\n", i + 1).as_bytes())?;
        for sk in &private_key {
            let sk_hex = hex::encode(sk);
            private_file.write_all(sk_hex.as_bytes())?;
            private_file.write_all(b"\n")?;
        }

        public_file.write_all(format!("Public Key {}\n", i + 1).as_bytes())?;
        for pk in &public_key {
            let pk_hex = hex::encode(pk);
            public_file.write_all(pk_hex.as_bytes())?;
            public_file.write_all(b"\n")?;
        }

    }

    Ok(())
}

pub fn verify_key_pair(private_key: &Vec<[u8; 32]>, public_key: &Vec<[u8; 32]>) -> bool {
    for (sk, pk) in private_key.iter().zip(public_key.iter()) {
        let computed_pk = Sha256::digest(sk);
        if &computed_pk[..] != pk {
            return false;
        }
    }
    true
}

pub fn read_keys_from_files(private_key_file: &str, public_key_file: &str) -> (Vec<String>, Vec<String>) {
    let private_file = File::open(private_key_file).expect("Unable to open private key file");
    let public_file = File::open(public_key_file).expect("Unable to open public key file");

    let private_keys: Vec<String> = BufReader::new(private_file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.starts_with("Private Key"))
        .collect();

    let public_keys: Vec<String> = BufReader::new(public_file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.starts_with("Public Key"))
        .collect();

    (private_keys, public_keys)
}

pub fn verify_encoded_key_pair(encoded_private_key: &Vec<String>, encoded_public_key: &Vec<String>) -> bool {
    for (sk_hex, pk_hex) in encoded_private_key.iter().zip(encoded_public_key.iter()) {
        let sk = hex::decode(sk_hex).expect("Invalid hex in private key");
        let pk = hex::decode(pk_hex).expect("Invalid hex in public key");

        let computed_pk = Sha256::digest(&sk);
        if computed_pk[..] != pk[..] {
            return false;
        }
    }
    true
}

pub fn sign_message(message: &str, private_key: &Vec<String>) -> Vec<String> {
    let mut signature = Vec::new();
    let message_hash = Sha256::digest(message.as_bytes());

    for (i, &byte) in message_hash.iter().enumerate().take(KEY_SIZE / 8) {
        for bit in 0..8 {
            let index = i * 8 + bit;
            let bit_value = (byte >> bit) & 1;
            let sk = &private_key[index * 2 + bit_value as usize];
            signature.push(sk.clone());
        }
    }

    signature
}

pub fn verify_signature(message: &str, signature: &Vec<String>, public_key: &Vec<String>) -> bool {
    let message_hash = Sha256::digest(message.as_bytes());

    for (i, &byte) in message_hash.iter().enumerate().take(KEY_SIZE / 8) {
        for bit in 0..8 {
            let index = i * 8 + bit;
            let bit_value = (byte >> bit) & 1;
            let sk = hex::decode(&signature[index]).expect("Invalid hex in signature");
            let pk = &public_key[index * 2 + bit_value as usize];
            
            let computed_pk = Sha256::digest(&sk);
            let expected_pk = hex::decode(pk).expect("Invalid hex in public key");

            if computed_pk[..] != expected_pk[..] {
                return false;
            }
        }
    }

    true
}