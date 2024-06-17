use bdk::bitcoin::secp256k1::rand;
use sha2::{Sha256, Digest};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_private_keys() -> Vec<String> {
    let path = "private.txt";
    let file = File::open(path).expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut private_keys = Vec::new();
    for line in reader.lines() {        
        private_keys.push(line.unwrap().to_string());
    }

    private_keys
}

pub fn get_public_keys() -> Vec<String> {
    let path = "public.txt";
    let file = File::open(path).expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut public_keys = Vec::new();
    for line in reader.lines() {        
        public_keys.push(line.unwrap().to_string());
    }

    public_keys
}

// Sign a 1-bit message using the secret key
pub fn sign_message(message: bool, secret_key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(if message { secret_key } else { &secret_key[1..] });
    hasher.finalize().into()
}

// Verify a signature using the public key
pub fn verify_signature(message: bool, signature: &[u8; 32], public_key: &[u8; 32]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(if message { public_key } else { &public_key[1..] });
    hasher.finalize().as_slice() == signature
}

