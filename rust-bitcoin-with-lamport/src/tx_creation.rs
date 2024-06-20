use bdk::bitcoin::secp256k1::PublicKey;
use bitcoin::address::Address;
use bitcoin::network::Network;
use bitcoin::{CompressedPublicKey, PrivateKey};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use rand::Error;
use std::str::FromStr;
    // 038bbfe47284165b5f554683a575aa0599c32b30ac86b2da2ea1fbe16271581b7b

pub fn get_address_from_public_key() -> Address {

    // Convert the public key string to a PublicKey object
    let pub_key: bitcoin::CompressedPublicKey = CompressedPublicKey::from_str("038bbfe47284165b5f554683a575aa0599c32b30ac86b2da2ea1fbe16271581b7b").unwrap();

    // Generate the Bitcoin address from the public key
    let address = Address::p2wpkh(&pub_key, Network::Regtest);

    address
}
