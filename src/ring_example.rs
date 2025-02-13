use ring::{rand, signature,hmac};
use ring::signature::KeyPair;

pub fn run_ring_example() {
    // Create a random number generator
    let rng = rand::SystemRandom::new();

    // Generate a new Ed25519 keypair
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

    // Create a message to sign
    let message = b"hello, ring";

    // Sign the message
    let signature = key_pair.sign(message);

    // Get the public key
    let public_key = key_pair.public_key();

    // Verify the signature
    let verification_result = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        public_key.as_ref(),
    )
    .verify(message, signature.as_ref());

    // Check if the verification succeeded
    match verification_result {
        Ok(_) => println!("ring: Signature verified!"),
        Err(_) => println!("ring: Signature verification failed!"),
    }
}

pub fn generate_hmac(key:&[u8],message:&[u8]) ->Vec<u8>{
    let key = hmac::Key::new(hmac::HMAC_SHA256,key) ;
    hmac::sign(&key, message).as_ref().to_vec() 
}