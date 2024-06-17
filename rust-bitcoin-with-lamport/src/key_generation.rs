use rand::Rng;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};
use sha2::{Sha256, Digest};
use hex;

const BIT_SIZE: usize = 4; // Кількість біт у повідомленні
const KEY_PAIRS_PER_BIT: usize = 2; // Кількість пар ключів на кожен біт (0 або 1)

pub fn generate_key_pair() -> (Vec<[u8; 16]>, Vec<[u8; 32]>) {
    let mut rng = rand::thread_rng();
    let mut private_key = Vec::new();
    let mut public_key = Vec::new();
    
    for _ in 0..BIT_SIZE {
        let mut sk0 = [0u8; 16];
        rng.fill(&mut sk0);
        let pk0 = Sha256::digest(&sk0);
        
        let mut sk1 = [0u8; 16];
        rng.fill(&mut sk1);
        let pk1 = Sha256::digest(&sk1);
        
        private_key.push(sk0);
        private_key.push(sk1);
        
        public_key.push(pk0.into());
        public_key.push(pk1.into());
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

        // Перевірка правильності генерованого ключа
        if verify_key_pair(&private_key, &public_key) {
            println!("Key pair {} is valid.", i + 1);
        } else {
            println!("Key pair {} is invalid.", i + 1);
        }
    }

    Ok(())
}

pub fn verify_key_pair(private_key: &Vec<[u8; 16]>, public_key: &Vec<[u8; 32]>) -> bool {
    for (i, sk) in private_key.iter().enumerate() {
        let computed_pk = Sha256::digest(sk);
        let expected_pk = &public_key[i / KEY_PAIRS_PER_BIT];
        if &computed_pk[..] != expected_pk {
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

pub fn sign_message(message: &str, private_key: &Vec<String>) -> Vec<String> {
    let mut signature = Vec::new();

    // Перетворюємо повідомлення у вектор бітів
    let message_bits: Vec<u8> = message.chars().map(|c| c.to_digit(2).unwrap() as u8).collect();

    for (i, &bit) in message_bits.iter().enumerate().take(BIT_SIZE) {
        let key_index = i * KEY_PAIRS_PER_BIT + bit as usize;
        let sk = &private_key[key_index];
        signature.push(sk.clone());
    }

    signature
}

pub fn verify_signature(message: &str, signature: &Vec<String>, public_key: &Vec<String>) -> bool {
    // Перетворюємо повідомлення у вектор бітів
    let message_bits: Vec<u8> = message.chars().map(|c| c.to_digit(2).unwrap() as u8).collect();

    for (i, &bit) in message_bits.iter().enumerate().take(BIT_SIZE) {
        let key_index = i * KEY_PAIRS_PER_BIT + bit as usize;
        let sk_hex = &signature[key_index];
        let pk_hex = &public_key[key_index / KEY_PAIRS_PER_BIT];

        let sk = hex::decode(sk_hex).expect("Invalid hex in signature");
        let pk = hex::decode(pk_hex).expect("Invalid hex in public key");

        let computed_pk = Sha256::digest(&sk);
        if computed_pk[..] != pk[..] {
            return false;
        }
    }

    true
}

fn main() -> std::io::Result<()> {
    let key_count = 1; // Приклад: згенерувати 1 пару ключів для тестування
    generate_and_write_key_pairs(key_count)?;

    // Читання ключів з файлів
    let (encoded_private_keys, encoded_public_keys) = read_keys_from_files("private_keys.txt", "public_keys.txt");

    // Приклад підпису та перевірки
    let message = "0101"; // Приклад повідомлення з 4 біт
    let signature = sign_message(message, &encoded_private_keys);

    if verify_signature(message, &signature, &encoded_public_keys) {
        println!("Signature is valid.");
    } else {
        println!("Signature is invalid.");
    }

    Ok(())
}
