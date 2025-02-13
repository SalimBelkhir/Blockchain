use ring::signature::Signature;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;
use solana_sdk::signature;
/* pub fn run_secp256k1_example() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);

    let secret_key = SecretKey::from_slice(&random_bytes).expect("Failed to create secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let message = Message::from_slice(&[0xab; 32]).expect("Failed to create message");
    let signature = secp.sign_ecdsa(&message, &secret_key);

    assert!(secp.verify_ecdsa(&message, &signature, &public_key).is_ok());

    println!("secp256k1: Signature verified!");
}*/

pub fn generate_wallet() -> (PublicKey, SecretKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    loop {
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);

        if let Ok(secret_key) = SecretKey::from_slice(&random_bytes) {
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            return (public_key, secret_key);
        } 
    }
}

pub fn sign_transaction(secret_key: &SecretKey,message:&Message) -> secp256k1::ecdsa::Signature {
    let secp = Secp256k1::new();
    let signature = secp.sign_ecdsa(&message, &secret_key);
    return signature ;
}

pub fn verify_transaction(message: &Message,public_key:&PublicKey,secret_key: &SecretKey) {
    let secp = Secp256k1::new() ;
    assert!(secp.verify_ecdsa(&message, &sign_transaction(&secret_key, &message), &public_key).is_ok());
}
