use bdk::bitcoin::network::message;
use bitcoin::io::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};
use sha2::{Sha256, Digest};
use hex;

pub fn get_sha256(data:String)->String{
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn key_generation()-> (String, String){
    let pub_key:String;
    let prv_key:String;

    let mut rng = thread_rng();
    prv_key = get_sha256((0..16)
    .map(|_| rng.sample(Alphanumeric) as char)
    .collect());
    pub_key = get_sha256(prv_key.clone());
    (prv_key, pub_key)
}
pub fn write_keys_to_file(message_len: i8, private_file_path: &str, pub_file_path:&str)->Result<(), Error>{
    let mut private_file = BufWriter::new(File::create(private_file_path)?);
    let mut public_file = BufWriter::new(File::create(pub_file_path)?);

    for _ in 0..message_len*2{
        let (prv_key, pub_key) = key_generation();
        writeln!(private_file, "{}", prv_key)?;
        writeln!(public_file, "{}", pub_key)?;
    }

    Ok(())
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

pub fn signature(message: bool)->Vec<String>{
    let (private_keys, _) = read_keys_from_files("private_keys.txt", "public_keys.txt");
    let mut sig: Vec<String> = vec!();
    if message {
        sig.push(private_keys[1].clone());
    }else {
        sig.push(private_keys[0].clone());
    }
    sig
}

pub fn verify_signature(message: bool, sig: Vec<String>)-> bool{
    let (_, public_keys) = read_keys_from_files("private_keys.txt", "public_keys.txt");
    for pk in sig.iter() {
        let check = get_sha256(pk.to_string());
        let int_bool = match message {
            true => 1,
            false => 0,
        };
        if check != public_keys[int_bool]{
            return false
        }
    }
    true
}

pub fn long_signature(message: Vec<bool>)->Vec<String>{
    let (private_keys, _) = read_keys_from_files("private_long_keys.txt", "public_long_keys.txt");
    let mut sig: Vec<String> = vec!();
    for i in 0..message.len(){
        if message[i] {
            sig.push(private_keys[2*i+1].clone());
        }else {
            sig.push(private_keys[2*i].clone());
        }
    }   
    sig
}
pub fn verify_long_signature(message: Vec<bool>, sig: Vec<String>)-> bool{
    let (_, public_keys) = read_keys_from_files("private_long_keys.txt", "public_long_keys.txt");
    
    for i in 0..message.len(){
        let check = get_sha256(sig[i].to_string());
        let int_bool = match message[i] {
            true => 1,
            false => 0,
        };
        if check != public_keys[2*i + int_bool]{
            return false
        }
    }
    true
}
