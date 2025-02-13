use ring_example::generate_hmac;
use secp256k1::Message;
use secp256k1_example::{generate_wallet, sign_transaction, verify_transaction};
use sha2::*;
use sha2_example::{hash_data, Blockchain};
use solana_example::create_transaction;

mod secp256k1_example;
mod ring_example;
mod sha2_example;
mod solana_example;

fn main() {
/* 
    println!("Running secp256k1 example...");
    secp256k1_example::run_secp256k1_example();*/

    /* 
    println!("\nRunning ring example...");
    ring_example::run_ring_example();

    println!("\nRunning Solana example...");
    solana_example::run_solana_example();*/

    println!("\n generating wallet...");
    let (public_key,secret_key)= generate_wallet();
    println!("\n{:?} : ",public_key );
    println!("\n{:?} : ",secret_key );

    let message_data = b"Hello, world!";

    // Hash the message using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(message_data);
    let message_hash = hasher.finalize();

    
    let message = Message::from_slice(&message_hash).expect("Failed to create message");
    println!("signature : {:?}",sign_transaction(&secret_key, &message));

    verify_transaction(&message, &public_key, &secret_key);

    let data = b"block of data" ;
    let hash = hash_data(data);
    println!("hash : {:?} ",hash);

    let mut blockchain =  Blockchain::new();
    blockchain.add_block("block 1 data".to_string());
    blockchain.add_block("block 2 data".to_string());

    println!("blockchain : {:#?} ",blockchain) ;//{:#?} for a pretty display 

    let transaction = create_transaction() ;
    println!("{:#?}",transaction) ; 


    let key = b"secret_key";
    let msg =b"a message";
    let hmac = generate_hmac(key, msg);
    println!("hmac = {:x?}",hmac); // print in hexadecimal 
}