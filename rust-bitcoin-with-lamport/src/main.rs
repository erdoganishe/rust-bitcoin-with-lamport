mod my_lamport_sig;

use my_lamport_sig::{create_lamport_signature, save_keypair};

fn main() {
    save_keypair(1);
}
