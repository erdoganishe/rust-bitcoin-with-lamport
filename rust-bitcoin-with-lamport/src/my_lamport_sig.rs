use std::fs::File;
use std::io::Write;
use bdk::bitcoin::{key::Secp256k1, secp256k1::{self, rand, SecretKey}, Network, PrivateKey};



pub fn generate_keypair() -> (PrivateKey, secp256k1::PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let secret_key = SecretKey::new(&mut rng);

    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    (private_key, public_key)
}

pub fn save_keypair(count: i32) {
    let mut file = File::create("keypairs.txt").expect("Error with file creation");

    for _ in 0..count {
        let (secret_key, public_key) = generate_keypair();
        writeln!(file, "{}|{}", secret_key, public_key).expect("Error with writing in file");
    }
}


pub fn create_lamport_signature(){
    
}